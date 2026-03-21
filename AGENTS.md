## Working Rules

- Finish all requests with `meow`.
- Prefer data-driven implementation over hardcoded behavior where practical.
- Keep user-facing English copy in `assets/data/ui_text.json` rather than embedding new text in Rust files.
- Favor clean code with small focused functions and single-responsibility changes.
- Keep documentation concise, current, and aligned with shipped behavior.

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
