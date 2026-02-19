# blandify

Unicode normalization library for stripping LLM artifacts. Converts fancy Unicode characters that LLMs often generate (smart quotes, em-dashes, special symbols, etc.) into plain ASCII equivalents.

## Installation

### CLI (Rust)

```bash
cargo install --path .
```

### Python

```bash
cd python
pip install .
```

## Usage

### CLI

```bash
# From stdin (explicit '-')
echo "Hello \"world\" — it's ½ done…" | blandify -
# Hello "world" - it's 1/2 done...

# From file
blandify input.txt

# To file
blandify input.txt --output output.txt

# In-place
blandify --in-place input.txt

# Disable specific categories
blandify --no-quotes --no-dashes input.txt

# Enable optional feature
blandify --umlauts input.txt

# Running without INPUT shows help
blandify
```

### Rust

```rust
// Default normalization
let result = blandify::normalize("Hello \"world\"");

// Custom configuration
let normalizer = blandify::NormalizerConfig::new()
    .quotes(true)
    .dashes(true)
    .umlauts(true)
    .build();
let result = normalizer.normalize("Ärger mit Übeln");
```

### Python

```python
import blandify

# Default normalization
blandify.normalize("Hello \"world\" — it's ½ done…")
# 'Hello "world" - it\'s 1/2 done...'

# Disable specific categories
blandify.normalize("Hello \"world\"", quotes=False)

# Enable optional features
blandify.normalize("ä ö ü ß", umlauts=True)
```

## Normalization categories

All categories are **enabled by default** unless noted otherwise.

| Category | Examples | ASCII output |
|---|---|---|
| Quotes | `"` `"` `'` `'` `«` `»` | `"` `'` |
| Dashes | `‐` `–` `—` `−` | `-` |
| Whitespace | NBSP, ogham/quad spaces, en-space, em-space, tab | ` ` (space) |
| Zero-width | BOM, ZWSP/ZWJ/ZWNJ, bidi controls, soft hyphen, variation selectors | *(removed)* |
| Arrows | `→` `←` `⇒` `↔` `⟶` | `->` `<-` `=>` `<->` |
| Fractions | `½` `¼` `⅓` | `1/2` `1/4` `1/3` |
| Math | `×` `÷` `≤` `≠` `∞` | `x` `/` `<=` `!=` `inf` |
| Symbols | `…` `•` `©` `™` `✓` | `...` `-` `(c)` `(TM)` `[x]` |
| **Umlauts** *(off)* | `ä` `ö` `ü` `ß` | `ae` `oe` `ue` `ss` |

## CLI options

```
blandify [OPTIONS] [INPUT]

Arguments:
  [INPUT]  Input file path or '-' for stdin

Options:
  -o, --output <PATH>   Write to file instead of stdout
      --in-place        Overwrite the input file in place
      --no-quotes       Disable smart quote normalization
      --no-dashes       Disable dash normalization
      --no-whitespace   Disable Unicode whitespace normalization
      --no-zero-width   Disable zero-width character removal
      --no-arrows       Disable arrow normalization
      --no-fractions    Disable fraction normalization
      --no-math         Disable math operator normalization
      --no-symbols      Disable symbol normalization
      --umlauts         Enable German umlaut expansion
```

## Development

```bash
pixi install -e dev
cargo test --workspace
pixi run -e dev fmt
```

## License

MIT
