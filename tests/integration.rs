use blandify::{normalize, Categories, NormalizerConfig};

// ─── Convenience function ────────────────────────────────────────────

#[test]
fn convenience_normalize_works() {
    assert_eq!(
        normalize("Hello \u{201C}world\u{201D} \u{2014} it\u{2019}s \u{00BD} done\u{2026}"),
        "Hello \"world\" - it's 1/2 done..."
    );
}

// ─── Category: QUOTES ────────────────────────────────────────────────

#[test]
fn quotes_smart_single() {
    assert_eq!(normalize("\u{2018}hi\u{2019}"), "'hi'");
}

#[test]
fn quotes_smart_double() {
    assert_eq!(normalize("\u{201C}hi\u{201D}"), "\"hi\"");
}

#[test]
fn quotes_guillemets() {
    assert_eq!(normalize("\u{00AB}hi\u{00BB}"), "\"hi\"");
}

#[test]
fn quotes_single_angle() {
    assert_eq!(normalize("\u{2039}hi\u{203A}"), "'hi'");
}

#[test]
fn quotes_low_9() {
    assert_eq!(normalize("\u{201A}hi\u{201E}"), "'hi\"");
}

// ─── Category: DASHES ────────────────────────────────────────────────

#[test]
fn dash_en() {
    assert_eq!(normalize("a\u{2013}b"), "a-b");
}

#[test]
fn dash_em() {
    assert_eq!(normalize("a\u{2014}b"), "a-b");
}

#[test]
fn dash_figure() {
    assert_eq!(normalize("a\u{2012}b"), "a-b");
}

#[test]
fn dash_horizontal_bar() {
    assert_eq!(normalize("a\u{2015}b"), "a-b");
}

#[test]
fn dash_minus_sign() {
    assert_eq!(normalize("a\u{2212}b"), "a-b");
}

#[test]
fn dash_hyphen() {
    assert_eq!(normalize("a\u{2010}b"), "a-b");
}

// ─── Category: WHITESPACE ────────────────────────────────────────────

#[test]
fn whitespace_nbsp() {
    assert_eq!(normalize("a\u{00A0}b"), "a b");
}

#[test]
fn whitespace_em_space() {
    assert_eq!(normalize("a\u{2003}b"), "a b");
}

#[test]
fn whitespace_thin_space() {
    assert_eq!(normalize("a\u{2009}b"), "a b");
}

#[test]
fn whitespace_ideographic() {
    assert_eq!(normalize("a\u{3000}b"), "a b");
}

#[test]
fn whitespace_ogham_space() {
    assert_eq!(normalize("a\u{1680}b"), "a b");
}

#[test]
fn whitespace_quads() {
    assert_eq!(normalize("a\u{2000}b"), "a b");
    assert_eq!(normalize("a\u{2001}b"), "a b");
}

// ─── Category: ZERO_WIDTH ────────────────────────────────────────────

#[test]
fn zero_width_space() {
    assert_eq!(normalize("a\u{200B}b"), "ab");
}

#[test]
fn zero_width_bom() {
    assert_eq!(normalize("\u{FEFF}hello"), "hello");
}

#[test]
fn zero_width_joiner() {
    assert_eq!(normalize("a\u{200D}b"), "ab");
}

#[test]
fn zero_width_directional_marks() {
    assert_eq!(normalize("a\u{200E}\u{200F}b"), "ab");
}

#[test]
fn zero_width_isolates_removed() {
    assert_eq!(normalize("a\u{2066}\u{2068}b\u{2069}"), "ab");
}

#[test]
fn zero_width_misc_invisible_removed() {
    assert_eq!(normalize("a\u{061C}b"), "ab");
    assert_eq!(normalize("a\u{034F}b"), "ab");
    assert_eq!(normalize("a\u{00AD}b"), "ab");
}

#[test]
fn zero_width_variation_selectors_removed() {
    assert_eq!(normalize("\u{2764}\u{FE0F}"), "\u{2764}");
}

// ─── Category: ARROWS ────────────────────────────────────────────────

#[test]
fn arrow_right() {
    assert_eq!(normalize("\u{2192}"), "->");
}

#[test]
fn arrow_left() {
    assert_eq!(normalize("\u{2190}"), "<-");
}

#[test]
fn arrow_double_right() {
    assert_eq!(normalize("\u{21D2}"), "=>");
}

#[test]
fn arrow_double_left() {
    assert_eq!(normalize("\u{21D0}"), "<=");
}

#[test]
fn arrow_bidirectional() {
    assert_eq!(normalize("\u{2194}"), "<->");
}

#[test]
fn arrow_double_bidirectional() {
    assert_eq!(normalize("\u{21D4}"), "<=>");
}

#[test]
fn arrow_long_forms() {
    assert_eq!(normalize("\u{27F5}"), "<-");
    assert_eq!(normalize("\u{27F6}"), "->");
    assert_eq!(normalize("\u{27F7}"), "<->");
}

// ─── Category: FRACTIONS ─────────────────────────────────────────────

#[test]
fn fraction_half() {
    assert_eq!(normalize("\u{00BD}"), "1/2");
}

#[test]
fn fraction_quarter() {
    assert_eq!(normalize("\u{00BC}"), "1/4");
}

#[test]
fn fraction_three_quarters() {
    assert_eq!(normalize("\u{00BE}"), "3/4");
}

#[test]
fn fraction_third() {
    assert_eq!(normalize("\u{2153}"), "1/3");
}

#[test]
fn fraction_eighth() {
    assert_eq!(normalize("\u{215B}"), "1/8");
}

// ─── Category: MATH ──────────────────────────────────────────────────

#[test]
fn math_multiply() {
    assert_eq!(normalize("3\u{00D7}4"), "3x4");
}

#[test]
fn math_divide() {
    assert_eq!(normalize("12\u{00F7}3"), "12/3");
}

#[test]
fn math_less_equal() {
    assert_eq!(normalize("x\u{2264}5"), "x<=5");
}

#[test]
fn math_greater_equal() {
    assert_eq!(normalize("x\u{2265}5"), "x>=5");
}

#[test]
fn math_not_equal() {
    assert_eq!(normalize("x\u{2260}y"), "x!=y");
}

#[test]
fn math_plus_minus() {
    assert_eq!(normalize("\u{00B1}5"), "+/-5");
}

#[test]
fn math_infinity() {
    assert_eq!(normalize("\u{221E}"), "inf");
}

// ─── Category: SYMBOLS ───────────────────────────────────────────────

#[test]
fn symbol_ellipsis() {
    assert_eq!(normalize("wait\u{2026}"), "wait...");
}

#[test]
fn symbol_bullet() {
    assert_eq!(normalize("\u{2022} item"), "- item");
}

#[test]
fn symbol_copyright() {
    assert_eq!(normalize("\u{00A9}"), "(c)");
}

#[test]
fn symbol_registered() {
    assert_eq!(normalize("\u{00AE}"), "(R)");
}

#[test]
fn symbol_trademark() {
    assert_eq!(normalize("\u{2122}"), "(TM)");
}

#[test]
fn symbol_degree() {
    assert_eq!(normalize("90\u{00B0}"), "90 deg");
}

#[test]
fn symbol_checkmark() {
    assert_eq!(normalize("\u{2713}"), "[x]");
}

#[test]
fn symbol_ballot_x() {
    assert_eq!(normalize("\u{2717}"), "[ ]");
}

// ─── Category: UMLAUTS ───────────────────────────────────────────────

#[test]
fn umlauts_off_by_default() {
    assert_eq!(normalize("ä ö ü ß"), "ä ö ü ß");
}

#[test]
fn umlauts_enabled() {
    let n = NormalizerConfig::new().umlauts(true).build();
    assert_eq!(n.normalize("ä ö ü ß"), "ae oe ue ss");
}

#[test]
fn umlauts_uppercase() {
    let n = NormalizerConfig::new().umlauts(true).build();
    assert_eq!(n.normalize("Ä Ö Ü"), "Ae Oe Ue");
}

#[test]
fn umlauts_full_sentence() {
    let n = NormalizerConfig::new().umlauts(true).build();
    assert_eq!(
        n.normalize("Ärger mit Ölförderung"),
        "Aerger mit Oelfoerderung"
    );
}

#[test]
fn tabs_expand_by_default() {
    assert_eq!(normalize("\tindent"), "  indent");
}

// ─── Config toggling ─────────────────────────────────────────────────

#[test]
fn disable_quotes_preserves_them() {
    let n = NormalizerConfig::new().quotes(false).build();
    assert_eq!(n.normalize("\u{201C}hi\u{201D}"), "\u{201C}hi\u{201D}");
}

#[test]
fn disable_dashes_preserves_them() {
    let n = NormalizerConfig::new().dashes(false).build();
    assert_eq!(n.normalize("a\u{2014}b"), "a\u{2014}b");
}

#[test]
fn only_specific_categories() {
    let n = NormalizerConfig::from_categories(Categories::QUOTES | Categories::DASHES).build();
    // Quotes and dashes should be normalized
    assert_eq!(n.normalize("\u{201C}a\u{2014}b\u{201D}"), "\"a-b\"");
    // But other categories should not (e.g., ellipsis stays)
    assert_eq!(n.normalize("\u{2026}"), "\u{2026}");
}

// ─── Combined behavior ──────────────────────────────────────────────

#[test]
fn full_sentence_normalization() {
    let result =
        normalize("Hello \u{201C}world\u{201D} \u{2014} it\u{2019}s \u{00BD} done\u{2026}");
    assert_eq!(result, "Hello \"world\" - it's 1/2 done...");
}

#[test]
fn ascii_passthrough() {
    let input = "Hello, world! This is plain ASCII text. 123 foo-bar_baz.";
    assert_eq!(normalize(input), input);
}

#[test]
fn empty_input() {
    assert_eq!(normalize(""), "");
}

#[test]
fn zero_width_chars_in_middle_of_words() {
    assert_eq!(
        normalize("he\u{200B}llo\u{200D} wo\u{200C}rld"),
        "hello world"
    );
}

#[test]
fn mixed_categories() {
    let result = normalize("\u{2022} Price: 3\u{00D7}4 = 12\u{00A0}\u{00A9} \u{2192} done\u{2026}");
    assert_eq!(result, "- Price: 3x4 = 12 (c) -> done...");
}
