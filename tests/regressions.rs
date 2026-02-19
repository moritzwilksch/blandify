use blandify::NormalizerConfig;

#[test]
fn crlf_input_normalizes_without_losing_structure() {
    let n = NormalizerConfig::new().build();
    let input = "Hello\u{00A0}\u{00A0}world\r\n\tfoo .\r\n";

    let out = n.normalize(input);
    assert_eq!(out, "Hello  world\r\n  foo .\r\n");
}

#[test]
fn final_newline_behavior_is_preserved() {
    let n = NormalizerConfig::new().build();
    assert_eq!(n.normalize("a\n"), "a\n");
    assert_eq!(n.normalize("a"), "a");
}

#[test]
fn long_line_normalizes_correctly() {
    let n = NormalizerConfig::new().build();
    let repeated = "x\u{00A0}\u{00A0}\u{2014}\t";
    let input = repeated.repeat(5000);

    let out = n.normalize(&input);
    assert!(!out.contains('\u{00A0}'));
    assert!(!out.contains('\u{2014}'));
    assert!(!out.contains('\t'));
    assert!(out.contains("x  -  "));
}

#[test]
fn key_config_combinations_table() {
    struct Case {
        name: &'static str,
        cfg: NormalizerConfig,
        expected: &'static str,
    }

    let input = "\tA\u{00A0}\u{00A0}B ä";
    let cases = [
        Case {
            name: "default",
            cfg: NormalizerConfig::new(),
            expected: "  A  B ä",
        },
        Case {
            name: "whitespace_off",
            cfg: NormalizerConfig::new().whitespace(false),
            expected: "\tA\u{00A0}\u{00A0}B ä",
        },
        Case {
            name: "umlauts_on",
            cfg: NormalizerConfig::new().umlauts(true),
            expected: "  A  B ae",
        },
        Case {
            name: "whitespace_off_umlauts_on",
            cfg: NormalizerConfig::new().whitespace(false).umlauts(true),
            expected: "\tA\u{00A0}\u{00A0}B ae",
        },
    ];

    for case in cases {
        let n = case.cfg.build();
        let out = n.normalize(input);
        assert_eq!(out, case.expected, "case failed: {}", case.name);
    }
}
