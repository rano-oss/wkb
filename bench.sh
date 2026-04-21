#!/usr/bin/env bash
set -euo pipefail

echo "══════════════════════════════════════════════════════════════"
echo "  WKB Benchmark Suite"
echo "══════════════════════════════════════════════════════════════"
echo ""

# ── 1. Speed Benchmarks (Criterion) ─────────────────────────────────
echo "▶ [1/3] Speed benchmarks (Criterion)"
echo "  Running: cargo bench --bench bench_speed"
echo ""
cargo bench --bench bench_speed 2>&1
echo ""

# ── 2. Memory Benchmarks ────────────────────────────────────────────
echo "▶ [2/3] Memory benchmarks"
echo ""

cargo build --example bench_memory --release 2>/dev/null

echo "  ── RSS measurement ──"
/usr/bin/time -v ./target/release/examples/bench_memory 2>&1 | \
    grep -E "(Maximum resident|wall clock)" || true
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

cargo build --example bench_size_wkb --release 2>/dev/null
cargo build --example bench_size_xkbcommon --release 2>/dev/null
cargo build --example bench_size_xkbcommon_dl --release 2>/dev/null

echo "  ── size(1) output ──"
printf "  %-30s %10s %10s %10s %10s\n" "Binary" "text" "data" "bss" "total"
printf "  %-30s %10s %10s %10s %10s\n" "------" "----" "----" "---" "-----"

for bin in bench_size_wkb bench_size_xkbcommon bench_size_xkbcommon_dl; do
    SIZE_OUT=$(size "target/release/examples/$bin" 2>/dev/null | tail -1)
    TEXT=$(echo "$SIZE_OUT" | awk '{print $1}')
    DATA=$(echo "$SIZE_OUT" | awk '{print $2}')
    BSS=$(echo "$SIZE_OUT" | awk '{print $3}')
    TOTAL=$(echo "$SIZE_OUT" | awk '{print $4}')
    printf "  %-30s %10s %10s %10s %10s\n" "$bin" "$TEXT" "$DATA" "$BSS" "$TOTAL"
done
echo ""

echo "  ── stripped sizes ──"
for bin in bench_size_wkb bench_size_xkbcommon bench_size_xkbcommon_dl; do
    FPATH="target/release/examples/$bin"
    if [ -f "$FPATH" ]; then
        ORIG=$(stat --format=%s "$FPATH" 2>/dev/null || stat -f%z "$FPATH" 2>/dev/null)
        cp "$FPATH" "${FPATH}.stripped"
        strip "${FPATH}.stripped" 2>/dev/null || true
        STRIPPED=$(stat --format=%s "${FPATH}.stripped" 2>/dev/null || stat -f%z "${FPATH}.stripped" 2>/dev/null)
        rm -f "${FPATH}.stripped"
        printf "  %-30s  original: %s  stripped: %s\n" "$bin" \
            "$(numfmt --to=iec-i --suffix=B "$ORIG" 2>/dev/null || echo "${ORIG}B")" \
            "$(numfmt --to=iec-i --suffix=B "$STRIPPED" 2>/dev/null || echo "${STRIPPED}B")"
    fi
done
echo ""

if command -v cargo-bloat &>/dev/null; then
    echo "  ── cargo bloat (top 15 functions per binary) ──"
    for bin in bench_size_wkb bench_size_xkbcommon bench_size_xkbcommon_dl; do
        echo ""
        echo "  --- $bin ---"
        cargo bloat --example "$bin" --release -n 15 2>/dev/null || true
    done
else
    echo "  (cargo-bloat not found — install with: cargo install cargo-bloat)"
fi

echo ""
echo "══════════════════════════════════════════════════════════════"
echo "  Done. Criterion HTML reports in: target/criterion/"
echo "══════════════════════════════════════════════════════════════"
