use pyo3::prelude::*;

/// Normalize text by replacing common Unicode artifacts with plain ASCII forms.
///
/// Args:
///     input: Text to normalize.
///     quotes: Normalize smart quotes and apostrophes.
///     dashes: Normalize Unicode dashes and minus signs.
///     whitespace: Normalize Unicode whitespace to ASCII spaces.
///     zero_width: Remove zero-width and directional markers.
///     arrows: Normalize arrow symbols.
///     fractions: Normalize vulgar fractions.
///     math: Normalize common math symbols.
///     symbols: Normalize general symbols like ellipsis and bullets.
///     umlauts: Transliterate German umlauts (lossy).
#[pyfunction]
#[pyo3(signature = (
    input,
    *,
    quotes=true,
    dashes=true,
    whitespace=true,
    zero_width=true,
    arrows=true,
    fractions=true,
    math=true,
    symbols=true,
    umlauts=false,
))]
fn normalize(
    input: &str,
    quotes: bool,
    dashes: bool,
    whitespace: bool,
    zero_width: bool,
    arrows: bool,
    fractions: bool,
    math: bool,
    symbols: bool,
    umlauts: bool,
) -> String {
    let is_default = quotes
        && dashes
        && whitespace
        && zero_width
        && arrows
        && fractions
        && math
        && symbols
        && !umlauts;

    if is_default {
        return blandify_core::normalize(input);
    }

    let config = blandify_core::NormalizerConfig::new()
        .quotes(quotes)
        .dashes(dashes)
        .whitespace(whitespace)
        .zero_width(zero_width)
        .arrows(arrows)
        .fractions(fractions)
        .math(math)
        .symbols(symbols)
        .umlauts(umlauts);

    config.build().normalize(input)
}

/// Python bindings for `blandify`.
#[pymodule]
fn blandify(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(normalize, m)?)?;
    Ok(())
}
