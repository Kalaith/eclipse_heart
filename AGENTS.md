## Workspace Instructions

This project uses the shared RustGames agent instructions in [`../AGENTS.md`](../AGENTS.md). Codex should read and apply that file when working here.

## Working Rules

- Finish all requests with `meow`.
- Prefer data-driven implementation over hardcoded behavior where practical.
- Keep user-facing English copy in `assets/data/ui_text.json` rather than embedding new text in Rust files.
- Favor clean code with small focused functions and single-responsibility changes.
- Keep documentation concise, current, and aligned with shipped behavior.

## Rust Clean Code Tips

- Prefer descriptive names that explain domain intent, especially for match flow, timing, and state transitions.
- Keep functions narrow in scope. If a function is doing setup, validation, resolution, and logging, split it.
- Prefer structs and enums that model game concepts directly over loosely related primitives and boolean combinations.
- Let the type system carry meaning. Use enums for phases, ownership, targets, and outcomes instead of magic strings or integers.
- Pass slices and references when ownership is not required. Avoid cloning just to satisfy a short-lived read path.
- Keep borrowing simple. Favor short mutable borrows, collect log data first when needed, then emit UI or event updates after mutation.
- Use helper functions to isolate repetitive state updates like growth changes, reveal handling, and encounter cleanup.
- Keep branching readable. If a `match` arm or `if` block becomes dense, move the logic into a named helper with a clear purpose.
- Prefer explicit state transitions over hidden side effects. When a turn, reaction window, or encounter state changes, make that transition obvious in code.
- Avoid panic-driven control flow. Use `Option` and `Result` for recoverable paths, and reserve `expect` for invariant checks that truly indicate a bug.
- Write comments sparingly and only to explain non-obvious intent, rules assumptions, or why a workaround exists.
- Keep UI code focused on presentation and input capture. Rule enforcement and game resolution should stay in engine or state helpers.
- When adding data fields, prefer ones that represent real game concepts and remove stale fields/helpers once they stop serving the model.
- Add or update focused tests alongside behavior changes, especially for timing order, progression thresholds, hidden information, and turn-state bugs.
- Run `cargo fmt` and relevant tests after Rust changes so the repo stays consistent and regressions are caught early.

## Documentation Workflow

At the start of a task:
- Read this file before making changes.
- Check whether the task affects player-facing behavior, controls, systems, or workflow.
- Identify which docs need to stay aligned, especially `IMPLEMENTED_SYSTEMS.md`, `polish_backlog.md`, and `tasks.md`.

During implementation:
- If new user-facing text is needed, add it to `assets/data/ui_text.json`.
- Keep code comments minimal and only where they clarify non-obvious logic.

Once the task is complete:
- Update `IMPLEMENTED_SYSTEMS.md` if shipped behavior changed.
- Update `polish_backlog.md` when a backlog item is completed or its status changes.
- Keep `tasks.md` focused on product/design follow-ups rather than implementation status.
