# Repository Guidelines

## Project Structure & Module Organization
The repository is a Rust workspace with a Python binding crate.

- Core Rust library and CLI: `src/` (`lib.rs`, `main.rs`, normalization modules)
- Rust tests: `tests/` (`cli.rs`, `integration.rs`, `regressions.rs`, `invariants.rs`)
- Python bindings crate: `python/src/lib.rs`
- Python package config and tests: `python/pyproject.toml`, `python/tests/`
- CI and automation: `.github/workflows/ci.yml`, `lefthook.yml`, `pixi.toml`

Keep normalization behavior in focused Rust modules and add tests alongside behavior changes.

## Build, Test, and Development Commands
Use `pixi` for a reproducible dev environment and `cargo` for core Rust flows.

- `pixi install -e dev`: install all dev tools (Rust, maturin, pytest, lefthook).
- `cargo test --workspace`: run all Rust tests.
- `pixi run -e dev test`: same Rust test suite via pixi task.
- `pixi run -e dev fmt`: run `cargo fmt --all --check`.
- `pixi run -e dev lefthook-run`: run pre-commit checks on all files.
- `cd python && pixi run maturin develop --uv && pixi run pytest tests/`: build extension and run Python tests.

## Coding Style & Naming Conventions
- Rust uses default `rustfmt` style (4-space indentation, idiomatic formatting).
- Use `snake_case` for functions/modules and descriptive test names (for example, `collapse_spaces_is_opt_in`).
- Keep public API names stable and explicit; avoid exposing internal binding types unintentionally.
- Prefer small, composable normalization steps over large monolithic transforms.

## Testing Guidelines
- Rust: `cargo test --workspace` with `assert_cmd`, `predicates`, and `tempfile` for CLI/integration behavior.
- Python: `pytest` in `python/tests/test_blandify.py`.
- Add regression tests for every new normalization rule and CLI flag.
- Name tests by behavior (`*_defaults`, `*_toggle`, `*_exits_non_zero`) and cover both enabled/disabled paths.

## Commit & Pull Request Guidelines
- Current history uses short, imperative commits (for example, `fix linting`, `add python bindings`).
- Prefer: `<verb> <scope>` with clear intent; avoid `WIP` in shared history.
- PRs should include a brief problem/solution summary.
- PRs should link the relevant issue when applicable.
- PRs should list tests added or updated.
- PRs should include CLI output snippets when flags or behavior change.
- Ensure CI passes (`fmt`, Rust tests, Python tests) before requesting review.
