#!/usr/bin/env node
// scripts/generate_benchmark_table.js
// Parses criterion JSON + memory/size output and generates a markdown table.
// Usage: node scripts/generate_benchmark_table.js [--update-readme]

const fs = require('fs');
const path = require('path');

const CRITERION_DIR = path.join(__dirname, '..', 'target', 'criterion');
const README = path.join(__dirname, '..', 'README.md');

// Only compare these three implementations
const IMPLS = ['wkb', 'xkbcommon', 'xkbcommon-dl'];

function fmt_ns(ns) {
    if (ns >= 1e6) return (ns / 1e6).toFixed(2) + ' ms';
    if (ns >= 1e3) return (ns / 1e3).toFixed(1) + ' µs';
    return ns.toFixed(0) + ' ns';
}

function speedup(base, other) {
    if (!base || !other) return '';
    const ratio = other / base;
    if (ratio >= 1.05) return `**${ratio.toFixed(1)}x faster**`;
    if (ratio <= 0.95) return `${(1 / ratio).toFixed(1)}x slower`;
    return '~same';
}

function readEstimates(groupDir, implName) {
    // Handle both flat (e.g. full_setup/wkb) and nested (e.g. key_update/wkb/us/plain_a)
    const flat = path.join(groupDir, implName, 'new', 'estimates.json');
    if (fs.existsSync(flat)) {
        const d = JSON.parse(fs.readFileSync(flat, 'utf8'));
        return d.median?.point_estimate || d.mean?.point_estimate;
    }
    // Aggregate nested benchmarks (take median of medians)
    const implDir = path.join(groupDir, implName);
    if (!fs.existsSync(implDir)) return null;
    const values = [];
    function walk(dir) {
        for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
            if (e.isDirectory()) walk(path.join(dir, e.name));
            else if (e.name === 'estimates.json' && dir.includes('/new')) {
                const d = JSON.parse(fs.readFileSync(path.join(dir, e.name), 'utf8'));
                const v = d.median?.point_estimate || d.mean?.point_estimate;
                if (v) values.push(v);
            }
        }
    }
    walk(implDir);
    if (values.length === 0) return null;
    values.sort((a, b) => a - b);
    return values[Math.floor(values.length / 2)];
}

function generateSpeedTable() {
    if (!fs.existsSync(CRITERION_DIR)) return null;

    const groups = fs.readdirSync(CRITERION_DIR)
        .filter(d => !d.startsWith('.') && d !== 'report');

    // Criterion stores groups with '_' separators in directory names
    const interesting = [
        { dir: 'full_setup',             label: 'Full setup' },
        { dir: 'key_update',             label: 'Key update' },
        { dir: 'key_get_utf8',           label: 'Get UTF-8' },
        { dir: 'key_get_sym',            label: 'Get keysym' },
        { dir: 'compose_setup',          label: 'Compose setup', combine: ['compose_table_creation', 'compose_state_creation'] },
        { dir: 'compose_feed',           label: 'Compose feed' },
    ];

    const rows = [];
    for (const { dir, label, combine } of interesting) {
        const vals = {};

        if (combine) {
            // Sum estimates from multiple benchmark groups
            for (const impl_ of IMPLS) {
                let total = 0;
                let found = false;
                for (const subDir of combine) {
                    const gpath = path.join(CRITERION_DIR, subDir);
                    const v = fs.existsSync(gpath) ? readEstimates(gpath, impl_) : null;
                    if (v != null) { total += v; found = true; }
                }
                vals[impl_] = found ? total : null;
            }
        } else {
            const gpath = path.join(CRITERION_DIR, dir);
            if (!fs.existsSync(gpath)) continue;
            for (const impl_ of IMPLS) {
                vals[impl_] = readEstimates(gpath, impl_);
            }
        }
        if (!vals['wkb']) continue;

        const row = [
            label,
            vals['wkb'] ? fmt_ns(vals['wkb']) : '-',
            vals['xkbcommon'] ? fmt_ns(vals['xkbcommon']) : '-',
            vals['xkbcommon-dl'] ? fmt_ns(vals['xkbcommon-dl']) : '-',
            speedup(vals['wkb'], vals['xkbcommon']),
        ];
        rows.push(row);
    }

    if (rows.length === 0) return null;

    let table = '| Benchmark | wkb | xkbcommon | xkbcommon-dl | vs xkbcommon |\n';
    table += '|-----------|-----|-----------|--------------|-------------|\n';
    for (const r of rows) {
        table += `| ${r.join(' | ')} |\n`;
    }
    return table;
}

function generateMemoryTable(memOutput) {
    if (!memOutput) return null;
    const lines = memOutput.trim().split('\n');
    const data = {};
    for (const line of lines) {
        const m = line.match(/^(\S+?)\/(after_workload)\s+RSS:\s+(\d+)\s+kB/);
        if (m) {
            data[m[1]] = parseInt(m[3]);
        }
    }
    if (Object.keys(data).length === 0) return null;

    let table = '| Library | Peak RSS |\n';
    table += '|---------|----------|\n';
    for (const impl_ of IMPLS) {
        const v = data[impl_] || data[impl_.replace('-', '_')];
        if (v) table += `| ${impl_} | ${(v / 1024).toFixed(1)} MB |\n`;
    }
    return table;
}

function generateSizeTable(sizeOutput) {
    if (!sizeOutput) return null;
    const lines = sizeOutput.trim().split('\n');
    const data = {};
    for (const line of lines) {
        const m = line.match(/^(\S+)\s+(\d+)/);
        if (m) data[m[1]] = parseInt(m[2]);
    }
    if (Object.keys(data).length === 0) return null;

    let table = '| Binary | Size (stripped) |\n';
    table += '|--------|----------------|\n';
    for (const [name, size] of Object.entries(data)) {
        table += `| ${name} | ${(size / 1024).toFixed(0)} KB |\n`;
    }
    return table;
}

function generateFullSection(memOutput, sizeOutput) {
    const date = new Date().toISOString().split('T')[0];
    let section = `*Last updated: ${date} (automated via CI)*\n\n`;

    const speedTable = generateSpeedTable();
    if (speedTable) {
        section += '### Speed\n\n' + speedTable + '\n';
    }

    if (memOutput) {
        const memTable = generateMemoryTable(memOutput);
        if (memTable) section += '### Memory\n\n' + memTable + '\n';
    }

    if (sizeOutput) {
        const sizeTable = generateSizeTable(sizeOutput);
        if (sizeTable) section += '### Binary Size\n\nSizes for xkbcommon and xkbcommon-dl include the dynamically-linked `libxkbcommon.so`.\n\n' + sizeTable + '\n';
    }

    return section;
}

function updateReadme(section) {
    let content = fs.readFileSync(README, 'utf8');
    const startMarker = '<!-- BENCHMARK_START -->';
    const endMarker = '<!-- BENCHMARK_END -->';

    const start = content.indexOf(startMarker);
    const end = content.indexOf(endMarker);

    if (start === -1 || end === -1) {
        console.error('README markers not found');
        process.exit(1);
    }

    const before = content.slice(0, start + startMarker.length);
    const after = content.slice(end);
    content = before + '\n' + section + after;

    fs.writeFileSync(README, content);
    console.log('README.md updated with benchmark results');
}

// Main
const memFile = process.argv.find(a => a.startsWith('--mem='));
const sizeFile = process.argv.find(a => a.startsWith('--size='));
const doUpdate = process.argv.includes('--update-readme');

const memOutput = memFile ? fs.readFileSync(memFile.split('=')[1], 'utf8') : null;
const sizeOutput = sizeFile ? fs.readFileSync(sizeFile.split('=')[1], 'utf8') : null;

const section = generateFullSection(memOutput, sizeOutput);

if (doUpdate) {
    updateReadme(section);
} else {
    console.log(section);
}
