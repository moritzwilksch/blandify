use blandify::{normalize, NormalizerConfig};

#[test]
fn normalization_is_idempotent_for_mixed_corpus() {
    let corpus = [
        "",
        "plain ascii",
        "Hello \u{201C}world\u{201D} \u{2014} \u{00BD}",
        "a\u{00A0}\u{00A0}b",
        "\tindent",
        "hello .\nline\u{00A0}\u{00A0}two  \n",
        "he\u{200B}llo\u{200D}\u{200C}",
        "x\u{2264}5 \u{2192} done\u{2026}",
    ];

    for input in corpus {
        let once = normalize(input);
        let twice = normalize(&once);
        assert_eq!(once, twice, "idempotence failed for input: {input:?}");
    }
}

#[test]
fn ascii_input_stays_ascii_for_default_config() {
    let ascii_samples = [
        "Hello, world! 123",
        "foo-bar_baz\nline2",
        "tabs\tbecome two spaces",
        "spacing   kept by default",
        "hello .\ntrail   \n",
    ];

    for input in ascii_samples {
        let output = normalize(input);
        assert!(
            output.is_ascii(),
            "non-ascii output for ascii input: {output:?}"
        );
    }
}

#[test]
fn umlaut_category_controls_transliteration() {
    let default_n = NormalizerConfig::new().build();
    let umlaut_n = NormalizerConfig::new().umlauts(true).build();

    assert_eq!(default_n.normalize("ä ö ü ß"), "ä ö ü ß");
    assert_eq!(umlaut_n.normalize("ä ö ü ß"), "ae oe ue ss");
}

#[test]
fn whitespace_category_controls_tab_expansion() {
    let default_n = NormalizerConfig::new().build();
    let no_ws = NormalizerConfig::new().whitespace(false).build();

    assert_eq!(default_n.normalize("\tindent"), "  indent");
    assert_eq!(no_ws.normalize("\tindent"), "\tindent");
}
