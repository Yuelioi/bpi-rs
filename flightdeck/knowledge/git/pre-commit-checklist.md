# Git pre-commit checklist
SUMMARY: Always run the repository pre-commit gate before committing; it combines GitHub Actions-equivalent checks with Git patch whitespace checks.
READ WHEN: before any commit in this repository.

---

Use `task pre_commit` before committing. It runs the same check set exposed by `task ci`, then runs both `git diff --check` and `git diff --cached --check`.

The extra Git checks matter because cargo can pass while Git still rejects or flags a patch for whitespace errors or conflict-marker style problems.

This repository also has `core.hooksPath` set to `.githooks`, with `.githooks/pre-commit` invoking `task pre_commit`.
