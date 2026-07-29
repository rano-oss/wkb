# Phase 2 report: reduce the intermediate XKB representation

Baseline: `a939868` (Phase 1). Phase 2 keeps `WKB`, its fields, its flat
runtime maps, its ownership model, and its public API unchanged.

## 1. What changed

- RMLVO component expansion now uses the byte-native `Vec<u8>` representation
  consumed by the XKB parser. The four component copies and the extra symbols
  copy used only to convert `i8` back to `u8` are gone.
- Geometry output is no longer expanded or retained by RMLVO resolution. The
  parser already explicitly discards geometry grammar values because geometry
  is unsupported and irrelevant to Wayland keyboard conversion.
- Symbol interpretations now live in the short-lived `XkbKeymapInfo` compiler
  context. They are used to derive actions, virtual modifiers, and repeat
  behavior, then dropped instead of being retained by the compiled
  `XkbKeymap`.
- Per-level action buffers are released after their effects have been derived
  and before conversion allocates the existing WKB flat tables.
- A general `explicit: u32` key bitmask was replaced by the only two flags
  still read after compilation: explicit repeat and explicit virtual-modifier
  mapping.
- Single-symbol levels no longer compute and retain an unused uppercase
  keysym. Multi-symbol uppercase expansion remains unchanged because conversion
  consumes those expanded symbols.

## 2. Old code deleted

The following write-only or post-compilation-unused data was removed:

- `XkbKeymap` alias markers, canonical-state mask, automatic redirect key,
  symbol interpretations, and four section-name strings.
- `required` flags on key types and symbol interpretations, including the
  canonical-type atom lookups used only to populate them.
- Final key overlay copies, key-level implicit-action state, group
  explicit-symbol/type flags, and level uppercase cache.
- Section-name ownership and merge plumbing from keycodes, types, compat, and
  symbols compiler contexts.
- The pass which counted and allocated empty key-alias markers.
- A redundant canonical-modifier accumulation pass.
- RMLVO `Vec<i8>` append helpers, conversion collections, and ignored geometry
  output construction.

Parsing, include handling, overlay validation, alias resolution, action
derivation, XKB serialization, and conversion into the existing WKB tables
remain in place.

## 3. Lines of code

| Metric | Phase 1 | Phase 2 | Change |
| --- | ---: | ---: | ---: |
| `wc -l` over `src/xkb/*.rs` | 46,295 | 46,060 | -235 |
| `tokei` Rust code | 45,178 | 44,948 | -230 |
| Code-patch additions/deletions | — | 82 / 317 | -235 net |

Unsafe-token count in `src/xkb` is unchanged at 5; Phase 2 adds no unsafe code.

## 4. Benchmarks

Setup and representative runtime comparisons below use clean Phase 1 and
Phase 2 trees, the same lockfile/toolchain, offline release builds, and CPU 0.
Values are Criterion midpoint estimates.

| Benchmark | Phase 1 | Phase 2 | Change |
| --- | ---: | ---: | ---: |
| Typical RMLVO setup | 3.6196 ms | 3.3905 ms | -6.33% |
| Typical XKB-string setup | 647.21 µs | 594.86 µs | -8.09% |
| Multi-layout RMLVO setup | 3.6824 ms | 3.3878 ms | -8.00% |
| Multi-layout XKB-string setup | 1.9531 ms | 1.9533 ms | +0.01% |
| Key update, plain `A` | 44.603 ns | 45.065 ns | +1.04% |
| Character lookup, plain `A` | 46.913 ns | 48.178 ns | +2.70% |
| Keysym lookup, plain `A` | 47.303 ns | 48.480 ns | +2.49% |

The complete unpinned `bench_key` sweep showed large run-to-run variance in
the shortest paths, including incompatible results from consecutive runs of
the same binary. Clean, pinned builds were therefore used for the comparison
above. All representative runtime changes remain below the 5% limit.

The complete `bench_compose` sweep was within noise overall. The representative
acute-E feed changed from the Phase 1 value of 138.06 ns to 135.90 ns
(-1.56%). Compose setup measured 5.480 ms versus 5.599 ms in Phase 1 (-2.13%).
No compose implementation was changed.

## 5. Allocations

There is no allocation-counting harness or supported heap profiler in the
repository environment, so peak bytes were not quantified.

Static inspection establishes that successful RMLVO compilation no longer
performs at least five conversion-only `Vec` allocations: four component
`i8`-to-`u8` collections and the symbols collection used to count explicit
layouts. It also removes section-name clones, ignored geometry expansion,
the key-alias marker allocation, and unnecessary single-level uppercase work.
Compiler-only interpretations and action capacities are dropped before WKB
table construction, reducing retained temporary memory at the conversion
boundary.

## 6. Binary size

The two requested bare `cargo bloat` commands still fail because this package
has only a library target. Using the same `bench_size_wkb` binary as Phase 1:

| Metric | Phase 1 | Phase 2 | Change |
| --- | ---: | ---: | ---: |
| WKB crate `.text` | 331.7 KiB | 327.0 KiB | -4.7 KiB (-1.42%) |
| Total `.text` | 612.9 KiB | 607.2 KiB | -5.7 KiB (-0.93%) |

The requested `--filter wayland_keyboard` spelling returns zero symbols for
the example because cargo-bloat labels the crate `wkb`; the crate summary is
the meaningful result.

## 7. Validation

- `cargo check --all-features`: passed
- `cargo test --all-features`: 5,935 passed, 0 failed
- Focused compile/keymap/layout/LED/modifier/repeat differential tests:
  1,262 passed, 0 failed
- `cargo bench --bench bench_setup`: passed
- `cargo bench --bench bench_key`: passed
- `cargo bench --bench bench_compose`: passed
- `cargo clippy --all-targets --all-features -- -D warnings`: completed but
  fails on 40 pre-existing warnings under Rust 1.93 (Phase 1 baseline: 41);
  Phase 2 introduces no new warning category
- `find ... | wc -l`, `tokei src/xkb`, and both requested cargo-bloat
  invocations: completed

## 8. Semantic risks and unsupported cases

- Section names and alias marker objects were write-only after compilation.
  Differential serialization and round-trip tests pass without them.
- Action vectors are discarded only after repeat, modifier, group-action, LED,
  and pending-action derivation succeeds. Action and state differential tests
  pass.
- Overlay syntax is still parsed, merged, and validated. Only the unused copy
  into the finalized compatibility key was removed.
- Geometry remains explicitly unsupported as before. Phase 2 avoids expanding
  the ignored RMLVO geometry result; it does not silently reinterpret geometry
  as keyboard semantics.
- No compiler structure was moved into WKB, and no WKB runtime change was made.

## 9. Recommended next phase

Evaluate compose-table caching only behind the existing compatibility/compose
loader boundary. If sharing a table would require changing any `WKB` field,
ownership, or runtime representation, defer it and proceed instead to the
dense atom-ID map audit. The architectural WKB boundary remains non-negotiable.
