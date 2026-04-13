---

name: safe-rust-refactor
description: use when refactoring rust pull requests or repositories to replace ffi, unsafe, raw pointers, or extern usage with standard safe rust while preserving behavior. prefer small reviewable patches, run only existing tests, do not add tests, avoid repeated clarification questions, and keep responses compact.
-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

# Behavior

Keep output compact.
Prefer short direct wording.
Assume user wants action, not discussion.

# Primary goal

Rewrite FFI- and unsafe-based code into standard safe Rust where feasible.

# Constraints

* Preserve existing behavior.
* Do not add new tests.
* Do not suggest adding more tests.
* Run the existing test suite only.
* If tests are slow, wait for them to finish rather than changing strategy.
* Do not repeatedly ask for confirmation unless blocked by a truly ambiguous API or behavior question.
* Prefer incremental refactors that reduce PR size and improve reviewability.
* Split work into small coherent commits or patches when possible.
* Keep responses brief and low-token.

# Execution rules

* First identify unsafe, extern, raw pointer, and FFI-heavy areas.
* Refactor the code in the smallest safe steps possible.
* After each meaningful step, run the existing relevant tests.
* Keep public pure rust APIs stable unless a change is required to eliminate unsafe or FFI usage.
* Avoid introducing new dependencies unless clearly necessary.
* Do not create new abstractions unless they simplify or replace unsafe or FFI logic.

# Non-goals

* No new test coverage.
* No speculative cleanup unrelated to removing FFI or unsafe.
* No architecture rewrite beyond what is needed to remove FFI or unsafe.

# Communication rules

* Be concise.
* Do not restate the goal repeatedly.
* Do not ask the user to re-explain priorities already given.
* Only stop for a question if behavior cannot be inferred from code and existing tests.
* When blocked, ask one specific question, not many.

# Default workflow

1. Inspect the PR or repo.
2. Find unsafe, FFI, raw pointer, and extern sections.
3. Refactor the smallest coherent section.
4. Run existing tests only.
5. Report briefly: changed, remaining, blocker if any.
6. Continue.

# Repo task template

When the user provides a PR, treat it as the source of truth.

Default interpretation:
"Refactor this PR so that all feasible FFI and unsafe code is rewritten to standard Rust while preserving behavior and running only the existing tests. No new tests. Slow test execution is acceptable."

# Branch default

If running inside a local repository and the user does not provide a PR URL, assume the current checked-out branch is the target refactor branch.
Do not ask for a PR link if the code is already present locally.
Inspect the current branch, identify FFI and unsafe usage, refactor in small safe steps, and run existing tests only.

# FFI elimination priority

Treat internal `extern "C"` usage as technical debt to remove, not preserve.

If both caller and callee are Rust, rewrite the interaction as normal Rust instead of keeping an FFI-style boundary.

Prefer:

* direct function calls over symbol-style indirection
* references and slices over raw pointers
* Rust structs and enums over C-shaped transport types
* ownership and borrowing over manual memory conventions

Keep `extern "C"` only when it is required for a real external ABI boundary.
