# Phase 1: precomputed key types and consolidated finalization

Baseline commit: `aa58e6d`  
Toolchain: Rust 1.93.0, Criterion 0.8.2, tokei 14.0.0

## 1. What changed

- Added a compact `CompiledType` table for all 256 combinations of XKB's eight
  real modifier bits. Each entry stores the selected level and consumed
  modifiers, so finalization no longer linearly searches type entries for every
  key/layout/state.
- Consolidated state characters, raw level characters, named keys, Caps Lock
  overrides, Num Lock overrides, and repeat bits into one traversal of finalized
  keys.
- Precomputed the eight base, Caps, and Num modifier states per layout.
- Preclassified modifier definitions once instead of resolving modifier names
  and masks again for every modifier key.
- Added focused Criterion cases for XKB-string setup, multi-layout RMLVO setup,
  and multi-layout XKB-string setup.

## 2. Old code deleted

- `build_lock_keymap`
- `caps_is_consumed`
- `key_affected_by_num`
- Separate Caps-affected, Num-affected, and repeat-key passes
- Construction-only `Keymap` lookup wrappers and the now-unused flat-map setter
- The now-unused `XkbKeymap::get_key_level` wrapper

The implementation and benchmark patch has 280 inserted and 415 deleted lines;
this report is excluded from those diff counts.

## 3. LOC

| Metric | Baseline | Phase 1 | Change |
| --- | ---: | ---: | ---: |
| `find src/xkb ... \| wc -l` | 46,457 | 46,295 | -162 |
| `tokei` Rust code in `src/xkb` | 45,311 | 45,178 | -133 |

## 4. Benchmarks

Criterion slope point estimates from the same host:

| Benchmark | Baseline | Phase 1 | Change |
| --- | ---: | ---: | ---: |
| RMLVO setup, `us` | 2.712 ms | 2.721 ms | +0.3% |
| XKB-string setup, `us` | 1.309 ms | 0.762 ms | -41.8% |
| RMLVO setup, `us,de,fr,ru` | 4.872 ms | 3.922 ms | -19.5% |
| XKB-string setup, `us,de,fr,ru` | 2.113 ms | 1.993 ms | -5.7% |
| Setup with compose | 5.540 ms | 5.599 ms | +1.1% |
| Key update, plain `a` | 84.45 ns | 75.45 ns | -10.7% |
| Character lookup, plain `a` | 47.56 ns | 43.87 ns | -7.8% |
| Keysym lookup, plain `a` | 46.95 ns | 44.58 ns | -5.1% |
| Compose feed, acute `e` | 139.92 ns | 138.06 ns | -1.3% |

Single-layout RMLVO and compose-enabled construction are statistically neutral.
The runtime lookup implementation did not change; its measured improvements
should be treated as run-to-run variance, but demonstrate no regression.

## 5. Allocations

No allocator-counting harness is present, so allocation count and peak bytes
were not quantified. The refactor removes the two per-key `Vec<bool>` Caps/Num
buffers and their full-key passes. It adds one contiguous compiled-type buffer
and one small per-layout state buffer.

## 6. Binary size

The two requested bare `cargo bloat` commands fail on both baseline and Phase 1
because this package has only a library target and cargo-bloat requires a
`bin`, `dylib`, or `cdylib`. Using `bench_size_wkb` as the selected binary:

| Metric | Baseline | Phase 1 | Change |
| --- | ---: | ---: | ---: |
| WKB crate `.text` | 330.7 KiB | 331.7 KiB | +1.0 KiB |
| Total `.text` | 611.6 KiB | 612.9 KiB | +1.3 KiB |

## 7. Validation

- `cargo test --all-features`: 5,935 passed, 0 failed
- Focused level/state differential tests: passed, including 824 state cases
- Focused Caps/Num/layout/modifier/repeat differential tests: passed
- `cargo bench --bench bench_setup`: passed
- `cargo bench --bench bench_key`: passed
- `cargo bench --bench bench_compose`: passed
- `cargo fmt --all -- --check`: passed
- `git diff --check`: passed
- `cargo clippy --all-targets --all-features -- -D warnings`: the clean baseline
  fails with 46 Rust 1.93 lint errors; Phase 1 fails with 41. No new lint
  category remains in this patch.

## 8. Semantic risks and unsupported cases

- `CompiledType` relies on the existing XKB invariant that real modifier state
  occupies the low eight bits (`MOD_REAL_MASK_ALL == 0xff`). Virtual modifiers
  are already resolved to real mappings before this stage.
- Existing group wrapping, consumed-Caps behavior, conditional lock-key
  activation, and group-0 Caps/Num sensitivity gating are preserved.
- No syntax, layout, action, LED, compose, repeat, or serialization support was
  removed. No dependency or unsafe-code change was introduced.

## 9. Recommended next phase

Proceed to Phase 2: separate immutable compiled keymap data from mutable keyboard
state, preserving the final tables produced here and making them shareable.
