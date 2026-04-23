#!/usr/bin/env bash
set -euo pipefail

BINS="bench_size_wkb bench_size_xkbcommon bench_size_xkbcommon_dl bench_size_xkbcommon_compat"
RESULTS_DIR="/tmp/wkb_bench_results"
mkdir -p "$RESULTS_DIR"

echo "══════════════════════════════════════════════════════════════"
echo "  WKB Benchmark Suite"
echo "══════════════════════════════════════════════════════════════"
echo ""

# ── 1. Speed Benchmarks (Criterion) ─────────────────────────────────
echo "▶ [1/3] Speed benchmarks (Criterion)"
echo "  Running: cargo bench --bench bench_speed"
echo ""
cargo bench --bench bench_speed 2>&1 | tee "$RESULTS_DIR/speed.txt"
echo ""

# ── 2. Memory Benchmarks ────────────────────────────────────────────
echo "▶ [2/3] Memory benchmarks"
echo ""

cargo build --example bench_memory --release 2>/dev/null

echo "  ── RSS measurement ──"
./target/release/examples/bench_memory 2>&1 | tee "$RESULTS_DIR/memory.txt"
echo ""

if command -v valgrind &>/dev/null; then
    echo "  ── Valgrind Massif (peak heap) ──"
    valgrind --tool=massif --pages-as-heap=no --stacks=yes \
        ./target/release/examples/bench_memory 2>&1 | \
        grep -E "^==" | tail -5 || true

    MASSIF_FILE=$(ls -t massif.out.* 2>/dev/null | head -1)
    if [ -n "${MASSIF_FILE:-}" ]; then
        echo "  Peak from massif:"
        ms_print "$MASSIF_FILE" 2>/dev/null | head -30 || true
        rm -f "$MASSIF_FILE"
    fi
    echo ""
else
    echo "  (valgrind not found — skipping massif)"
    echo ""
fi

# ── 3. Binary Size ──────────────────────────────────────────────────
echo "▶ [3/3] Binary size comparison"
echo ""

for bin in $BINS; do
    cargo build --example "$bin" --release 2>/dev/null
done

echo "  ── size(1) output ──"
printf "  %-35s %10s %10s %10s %10s\n" "Binary" "text" "data" "bss" "total"
printf "  %-35s %10s %10s %10s %10s\n" "---" "----" "----" "---" "-----"

for bin in $BINS; do
    SIZE_OUT=$(size "target/release/examples/$bin" 2>/dev/null | tail -1)
    TEXT=$(echo "$SIZE_OUT" | awk '{print $1}')
    DATA=$(echo "$SIZE_OUT" | awk '{print $2}')
    BSS=$(echo "$SIZE_OUT" | awk '{print $3}')
    TOTAL=$(echo "$SIZE_OUT" | awk '{print $4}')
    printf "  %-35s %10s %10s %10s %10s\n" "$bin" "$TEXT" "$DATA" "$BSS" "$TOTAL"
done
echo ""

echo "  ── stripped sizes ──"
{
printf "  %-35s %12s %12s\n" "Binary" "original" "stripped"
printf "  %-35s %12s %12s\n" "---" "--------" "--------"
for bin in $BINS; do
    FPATH="target/release/examples/$bin"
    if [ -f "$FPATH" ]; then
        ORIG=$(stat --format=%s "$FPATH" 2>/dev/null || stat -f%z "$FPATH" 2>/dev/null)
        cp "$FPATH" "${FPATH}.stripped"
        strip "${FPATH}.stripped" 2>/dev/null || true
        STRIPPED=$(stat --format=%s "${FPATH}.stripped" 2>/dev/null || stat -f%z "${FPATH}.stripped" 2>/dev/null)
        rm -f "${FPATH}.stripped"
        ORIG_H=$(numfmt --to=iec-i --suffix=B "$ORIG" 2>/dev/null || echo "${ORIG}B")
        STRIP_H=$(numfmt --to=iec-i --suffix=B "$STRIPPED" 2>/dev/null || echo "${STRIPPED}B")
        printf "  %-35s %12s %12s\n" "$bin" "$ORIG_H" "$STRIP_H"
    fi
done
} | tee "$RESULTS_DIR/binary_size.txt"
echo ""

if command -v cargo-bloat &>/dev/null; then
    echo "  ── cargo bloat (top 15 functions per binary) ──"
    for bin in $BINS; do
        echo ""
        echo "  --- $bin ---"
        cargo bloat --example "$bin" --release -n 15 2>/dev/null || true
    done
else
    echo "  (cargo-bloat not found — install with: cargo install cargo-bloat)"
fi

# ════════════════════════════════════════════════════════════════════════
# SUMMARY TABLE
# ════════════════════════════════════════════════════════════════════════
echo ""
echo ""
echo "══════════════════════════════════════════════════════════════════════════════"
echo "  SUMMARY"
echo "══════════════════════════════════════════════════════════════════════════════"
echo ""

# -- Speed summary: extract median times from Criterion output
echo "  ── Speed (median from Criterion) ──"
echo ""
printf "  %-50s %18s %18s %18s %18s\n" "Benchmark" "wkb" "xkbcommon" "xkbcommon-dl" "xkbcommon-compat"
printf "  %-50s %18s %18s %18s %18s\n" \
    "$(printf '%0.s─' {1..50})" \
    "$(printf '%0.s─' {1..18})" \
    "$(printf '%0.s─' {1..18})" \
    "$(printf '%0.s─' {1..18})" \
    "$(printf '%0.s─' {1..18})"

# Parse Criterion output: lines like "group/backend   time:   [low med high]"
# We capture the group name when we see "Benchmarking group/backend" and the median from "time:" lines
python3 -c "
import re, sys, collections

results = collections.defaultdict(dict)  # {bench_group: {backend: median}}
current = None

for line in open('$RESULTS_DIR/speed.txt'):
    # Match: 'group/backend'  (from Benchmarking line)
    m = re.match(r'^Benchmarking\s+(.+)', line.strip())
    if m:
        current = m.group(1).strip()
        continue
    # Match: time:   [1.234 us 1.345 us 1.456 us]
    m = re.search(r'time:\s+\[[\d.]+ \w+ ([\d.]+) (\w+)', line)
    if m and current:
        median = m.group(1) + ' ' + m.group(2)
        # Split current into group + backend
        parts = current.rsplit('/', 1)
        if len(parts) == 2:
            group, backend = parts
        else:
            group = current
            backend = '?'
        results[group][backend] = median
        current = None

for group in sorted(results):
    wkb = results[group].get('wkb', '-')
    xkb = results[group].get('xkbcommon', '-')
    dl = results[group].get('xkbcommon-dl', '-')
    compat = results[group].get('xkbcommon-compat', '-')
    print(f'  {group:<50s} {wkb:>18s} {xkb:>18s} {dl:>18s} {compat:>18s}')
" 2>/dev/null || echo "  (could not parse speed results — check $RESULTS_DIR/speed.txt)"
echo ""

# -- Memory summary
echo "  ── Memory (peak RSS in kB) ──"
echo ""
printf "  %-20s %12s\n" "Backend" "RSS (kB)"
printf "  %-20s %12s\n" "---" "--------"
grep "after_workload" "$RESULTS_DIR/memory.txt" 2>/dev/null | while read -r line; do
    LABEL=$(echo "$line" | awk -F/ '{print $1}' | xargs)
    RSS=$(echo "$line" | awk '{print $NF}')
    printf "  %-20s %12s\n" "$LABEL" "$RSS"
done
echo ""

# -- Binary size summary
echo "  ── Binary Size (stripped) ──"
echo ""
grep -v "^  ---" "$RESULTS_DIR/binary_size.txt" 2>/dev/null | grep -v "Binary" | while read -r line; do
    echo "$line"
done
echo ""

echo "══════════════════════════════════════════════════════════════════════════════"
echo "  Criterion HTML reports: target/criterion/"
echo "  Raw results:            $RESULTS_DIR/"
echo "══════════════════════════════════════════════════════════════════════════════"
