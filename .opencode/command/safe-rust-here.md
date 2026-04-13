---

## description: Refactor the current Rust branch to remove FFI and unsafe in small reviewable chunks.

Use context-mode.
Use compact cave-man style.
Apply safe-rust-refactor.

Work on current checked-out branch only.

Task:
Rewrite FFI and unsafe to safe standard Rust where feasible.
Preserve behavior.
Run existing tests only.
Do not add tests.
Slow tests fine.
Work in small reviewable chunks.
Do not keep asking questions.
Do not ask me for the PR URL.
Assume current branch is the target.
