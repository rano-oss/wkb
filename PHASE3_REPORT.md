# Phase 3 report: cache XKB compose compilation

Phase 3 caches compose compilation entirely inside the feature-gated XKB
compatibility layer. `WKB`, `Composer`, their fields, their ownership, and the
runtime keyboard-state representation are unchanged.

## 1. What changed

- Added a process-wide, thread-safe cache keyed by canonical compose-file path.
- The cached value is a pristine compiled `Composer` template behind `Arc`.
  Each load deep-clones the original owned `Composer`, so cursor and buffer
  progress remain instance-local without changing the runtime representation.
- Cache lookup uses a mutex-protected small vector. The expected number of
  compose paths is tiny, and this was 5 KiB smaller in release `.text` than a
  `HashMap` implementation.
- Parsing and cloning happen outside the mutex. The lock covers only lookup and
  insertion.
- Concurrent cold loads use the first successfully inserted template.
- Requested aliases are associated with their canonical path after the first
  lookup, avoiding repeated canonicalization on the warm path.
- Compose parsing now reports whether the root and every included file loaded
  successfully. Partial or failed loads preserve their previous immediate
  result but are not cached, allowing later attempts to succeed.
- The temporary token capacity is nine: up to eight parsed key characters plus
  a possible `Multi_key` token.
- Added Criterion cases that isolate uncached compose parsing from repeated
  construction using the same path.
- Added tests for canonical aliases, cross-thread reuse, failed-load recovery,
  and independent WKB compose progress.

With the `xkb` feature disabled, the cache and all its synchronization types
are absent. `cargo check --no-default-features` passes. There is no source diff
in `src/composer.rs` or `src/lib.rs`.

## 2. Old code removed

The old `load_compose_from_path` path parsed the compose file and all includes
for every WKB construction. It has been replaced by:

1. one cold parse into the existing `Composer`,
2. insertion of a pristine template into the XKB-only cache, and
3. warm deep clones into unchanged instance-local composers.

No runtime table, WKB field, serde shape, or public WKB API was removed or
changed.

## 3. LOC

| Metric | Phase 2 | Phase 3 | Change |
| --- | ---: | ---: | ---: |
| `src/xkb` physical Rust lines (`wc`) | 46,060 | 46,121 | +61 (+0.13%) |
| `src/xkb` Rust code (`tokei`) | 44,948 | 45,002 | +54 (+0.12%) |
| `src/composer.rs` physical lines | 137 | 137 | 0 |
| Unsafe tokens in `src/xkb` | 5 | 5 | 0 |

Production XKB changes are approximately LOC-neutral relative to the 46 KLOC
implementation. The larger test diff is not included in the production count.

## 4. Benchmarks

All controlled comparisons used the same lockfile and toolchain, offline
release builds, and CPU 0. Values are Criterion midpoint estimates.

| Benchmark | Phase 2 | Phase 3 | Change |
| --- | ---: | ---: | ---: |
| Repeated same-path compose load | 2.5658 ms | 87.292 us | -96.60% (29.4x faster) |
| Compose-enabled WKB setup | 5.5815 ms | 2.9200 ms | -47.68% (1.91x faster) |
| WKB setup without compose | 2.7509 ms | 2.6719 ms | -2.87% |
| Cold compose parse | about 2.66 ms | 2.6311 ms | within noise |
| Compose feed, acute-E | 72.508 ns | 72.279 ns | -0.32% |
| Key update, plain `A` | 45.204 ns | 47.997 ns | +6.18% |
| Character lookup, plain `A` | 47.734 ns | 49.275 ns | +3.23% |
| Keysym lookup, plain `A` | 47.627 ns | 50.203 ns | +5.41% |

The direct warm-load target is exceeded substantially. End-to-end
compose-enabled WKB construction improves by 1.91x, narrowly missing the 2x
goal because the unchanged `Composer` must still deep-clone its owned trie.
Making construction an `Arc` clone would cross the explicit WKB/Composer
runtime boundary and was not implemented.

The long complete benchmark sweep showed large host-speed shifts, including
the same final key-update binary measuring both 48 and 90 ns. Back-to-back
filtered runs are reported above. `WKB::update_key` has the same 172-instruction
generated body in the Phase 2 and Phase 3 binaries; only relocated call and
jump-table addresses differ. Nevertheless, the stable key-update sample is
6.18% slower and the keysym sample is 5.41% slower, so the requested 5%
guardrail is not claimed as met. No WKB runtime change was made to tune those
results.

The complete required benchmark binaries all exited successfully:

- `cargo bench --bench bench_setup`
- `cargo bench --bench bench_key`
- `cargo bench --bench bench_compose`

## 5. Allocations

There is no allocation-counting harness or supported heap profiler in the
repository environment, so allocation counts and peak bytes were not
quantified.

Static behavior is clear:

- warm construction no longer allocates or rebuilds parser-side compose
  entries while scanning UTF-8 text and includes;
- one compiled template is retained per successful canonical path, plus cheap
  path aliases;
- each WKB still owns the same `Composer` trie as before and therefore retains
  its deep-clone allocations;
- failed and incomplete loads do not retain a template.

This is intentionally less allocation reduction than an Arc-backed runtime
composer, which is outside the architectural boundary.

## 6. Binary size

The requested bare `cargo bloat` commands still fail because the package has
only a library target. The established `bench_size_wkb` example gives:

| Metric | Phase 2 | Phase 3 | Change |
| --- | ---: | ---: | ---: |
| WKB crate `.text` | 327.0 KiB | 332.1 KiB | +5.1 KiB (+1.56%) |
| Total `.text` | 607.2 KiB | 613.4 KiB | +6.2 KiB (+1.02%) |

`cargo bloat --filter wayland_keyboard` still reports zero symbols because the
crate is labelled `wkb`. No dependency was added.

## 7. Validation

- `cargo test --all-features`: 5,938 passed, 0 failed.
- `cargo test --test compose`: 120 passed, 0 failed.
- Existing xkbcommon differential export and behavior tests passed.
- `cargo check --all-features`: passed.
- `cargo check --no-default-features`: passed with the branch's existing
  dead-code warnings.
- `cargo clippy --all-targets --all-features -- -D warnings`: still fails on
  the same 40 pre-existing warnings recorded in Phase 2; no error points to a
  Phase 3 file.
- `git diff --check`: passed.
- All three required benchmark binaries: passed.
- Both requested bare cargo-bloat invocations: attempted and failed for the
  existing library-only-target reason.
- Example-target cargo-bloat measurements: passed.

## 8. Semantic risks and unsupported cases

- A successfully cached path is treated as stable for the process lifetime.
  Replacing a compose file at the same path does not invalidate the template.
  System compose files and locale mappings are normally stable after startup.
- The cache is strong and unbounded. In normal use the number of locale compose
  paths is very small; a larger eviction framework would add code and
  synchronization without measured benefit.
- A load with a missing or invalid root/include is deliberately not cached.
- Poisoned mutexes recover their contained cache rather than disabling compose.
- Because `Composer` is unchanged, WKB instances use the shared template to
  construct independent owned tries rather than retaining an Arc themselves.
- No syntax support was removed and no unsupported case was added in this
  phase.

## 9. Recommended next phase

Proceed to the dense-integer map audit. Replace atom-ID maps only where density
and bounds are demonstrated, benchmark compound-key hashing before selecting
an alternative, and keep all changes inside the XKB compatibility/compiler
layer.
