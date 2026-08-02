# Combined Phase 5 and 6 report: simplify AST ownership and expressions

Phases 5 and 6 were combined at the maintainer's request. The work is confined
to the XKB compatibility/compiler layer. `WKB`, `Composer`, their ownership,
their fields, the flat runtime maps, and the public API are unchanged.

## 1. What changed

- Replaced the thread-local bump arena and custom pointer wrapper with ordinary
  Rust ownership.
- Parser statement enums and parser-stack values now own their payloads
  directly. Only recursive expression edges and the parser's root-file handoff
  use `Box`.
- Changed composite file lists from `Vec<Box<XkbFile>>` to `Vec<XkbFile>`.
- Introduced `BinaryOp` and `UnaryOp` and made expression nodes carry those
  enums directly.
- Rewrote boolean, integer, string, enum, mask, modifier, level, and left-hand
  side resolution to return `Option<T>` instead of writing through output
  parameters and returning a separate success flag.
- Kept integer and mask evaluators separate where XKB arithmetic and mask
  semantics differ, while retaining their shared identifier lookup path.
- Arithmetic overflow, division overflow, and division by zero now produce an
  expression mismatch instead of risking a debug-build panic.
- Removed the last production `unsafe` code from `src/xkb`.
- Brought the complete all-target Clippy run from 40 baseline errors to zero.
  Small test-only lint fixes preserve the same test matrices and assertions.

## 2. Old code deleted

- `ArenaBox`, its raw non-null pointer storage, `Deref` implementations, and
  unsafe `Send`/`Sync` implementations.
- The thread-local `RefCell<Bump>`, arena allocation/reset helpers, and all
  parser-entry arena resets.
- The direct `bumpalo` dependency.
- Per-statement and parser-stack arena allocation calls.
- Numeric `STMT_EXPR_*` constants and the `ExprKind::stmt_type` conversion
  layer.
- C-style expression resolver output parameters and the associated temporary
  initialization/branch boilerplate.
- Redundant boxed elements inside composite-file vectors.

## 3. LOC

| Metric | Phase 3 baseline | Phase 5/6 | Change |
| --- | ---: | ---: | ---: |
| `src/xkb` physical Rust lines (`wc`) | 46,121 | 45,633 | -488 (-1.06%) |
| `src/xkb` Rust code (`tokei`) | 45,002 | 44,530 | -472 (-1.05%) |
| Unsafe tokens in `src/xkb` | 5 | 0 | -5 (-100%) |

The production diff is 650 insertions and 1,139 deletions across `Cargo.toml`
and `src/xkb`, a net deletion of 489 diff lines. Test-only lint changes are
excluded from the XKB LOC totals.

## 4. Benchmarks

The setup comparison used Phase 3 commit `fdc75ac` and the final tree in
separate builds with the same lockfile, toolchain, CPU 0, and Criterion
configuration. Values are midpoint estimates from immediate back-to-back
runs.

| Benchmark | Phase 3 | Phase 5/6 | Change |
| --- | ---: | ---: | ---: |
| Typical RMLVO setup | 3.5476 ms | 3.2230 ms | -9.15% |
| XKB-string setup | 649.36 us | 633.33 us | -2.47% |
| Four-layout RMLVO setup | 3.7000 ms | 3.6588 ms | -1.11% |
| Four-layout XKB-string setup | 1.9682 ms | 2.0105 ms | +2.15% (within noise) |
| Compose-enabled setup | 2.8966 ms | 2.9778 ms | +2.80% (within noise) |

Runtime and compose comparisons use the stable Phase 3 measurements and the
final CPU-0 sweep. No WKB or Composer runtime code changed.

| Benchmark | Phase 3 | Phase 5/6 | Change |
| --- | ---: | ---: | ---: |
| Key update, plain `A` | 47.997 ns | 42.509 ns | -11.43% |
| Character lookup, plain `A` | 49.275 ns | 44.812 ns | -9.06% |
| Named-key lookup, plain `A` | 50.203 ns | 49.804 ns | -0.79% |
| Compose feed, acute-E | 72.279 ns | 70.949 ns | -1.84% |
| Repeated same-path compose load | 87.292 us | 82.880 us | -5.05% |
| Cold compose parse | 2.6311 ms | 3.0802 ms | +17.07% |

The cold-compose regression is reported rather than attributed to this work:
the compose parser/cache was not changed, and other compose-feed cases ranged
from small improvements to small regressions during the same noisy sweep. The
full setup and compose benchmark binaries exited successfully. The complete
key benchmark was interrupted after all update cases and part of the character
cases; the missing representative named-key case was then run twice to obtain
the stable result above.

## 5. Allocations

A temporary counting global allocator measured ten compose-disabled
constructions per case. The serialized keymap used by the string case was
created before counters were reset.

| Case | Metric | Phase 3 | Phase 5/6 | Change |
| --- | --- | ---: | ---: | ---: |
| RMLVO | allocation calls | 9,002 | 8,927 | -75 (-0.83%) |
| RMLVO | requested bytes | 1,986,845 | 2,649,069 | +662,224 (+33.33%) |
| XKB string | allocation calls | 2,831 | 2,590 | -241 (-8.51%) |
| XKB string | requested bytes | 377,278 | 428,814 | +51,536 (+13.66%) |

Allocation calls fell because statement/parser-stack payloads no longer need
arena entries and composite-file elements no longer need boxes. Requested
bytes increased because direct enum payloads make AST vectors wider than
vectors of arena pointers. Peak live bytes were not measured. This memory
tradeoff is retained because parsing and RMLVO setup improved, source and
unsafe code decreased materially, and reintroducing per-node indirection would
reverse the allocation-count and ownership simplifications.

## 6. Binary size

The requested bare `cargo bloat` commands still fail because the package has
only a library target. The established `bench_size_wkb` example gives:

| Metric | Phase 3 | Phase 5/6 | Change |
| --- | ---: | ---: | ---: |
| WKB crate `.text` | 332.1 KiB | 331.6 KiB | -0.5 KiB (-0.15%) |
| Total `.text` | 613.4 KiB | 613.2 KiB | -0.2 KiB (-0.03%) |

## 7. Validation

- `cargo test --all-features`: 5,938 passed, 0 failed.
- Existing xkbcommon differential export, compile, type, level, modifier,
  layout, action, LED, repeat, and state tests passed.
- `cargo clippy --all-targets --all-features -- -D warnings`: passed.
- `cargo check --all-features`: passed.
- `cargo check --no-default-features`: passed with the existing feature-off
  dead-code warnings.
- `cargo fmt --all`: passed.
- `git diff --check`: passed.
- `cargo bench --bench bench_setup`: passed.
- `cargo bench --bench bench_compose`: passed.
- `cargo bench --bench bench_key`: broad run interrupted; all update cases,
  representative character lookup, and a separately repeated named-key lookup
  completed without failures.
- Both requested bare cargo-bloat commands were attempted and failed for the
  existing library-only-target reason; example-target bloat passed.

## 8. Semantic risks and unsupported cases

- Expression arithmetic now rejects the `i64::MIN / -1` and `-i64::MIN`
  overflow cases instead of panicking in debug builds. This is an explicit
  robustness improvement for invalid input.
- Direct AST payloads increase requested temporary bytes despite reducing
  allocation calls. Very large synthetic keymaps may therefore have a higher
  parsing-memory high-water mark; peak memory was not measured.
- Parser action values still transfer owned expression subtrees into pending
  computations where XKB group resolution requires deferred evaluation.
  Differential action and state tests cover this boundary.
- No XKB syntax was removed, no unsupported feature was newly accepted, and no
  WKB/Composer/runtime representation changed.

## 9. Recommended next phase

Pause compose/multi-keyboard architecture work until its ownership boundary is
decided. If compiler work continues, the next useful step is an isolated audit
of dense atom-ID maps (the earlier Phase 4 proposal), with allocation-byte and
setup benchmarks used to reject changes that merely trade hashes for wider
temporary storage.
