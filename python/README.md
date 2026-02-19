# blandify (Python bindings)

Python bindings for the `blandify` Rust Unicode normalization library.

## What it does

`blandify.normalize(...)` replaces common Unicode artifacts with plain ASCII forms, including:

- smart quotes and apostrophes
- Unicode dashes and minus signs
- non-ASCII whitespace (including tab expansion to two spaces)
- zero-width and directional markers
- arrows, vulgar fractions, common math symbols, and common text symbols
- optional German umlaut transliteration (`ä -> ae`, `ö -> oe`, `ü -> ue`, `ß -> ss`)

## Development

From the repository root:

```bash
cd python
pixi run maturin develop --uv
pixi run pytest tests/
```

Or from the root with the configured task:

```bash
pixi run -e dev python-test
```
