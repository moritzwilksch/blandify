use crate::categories::Categories;
use crate::config::NormalizerConfig;

/// The result of replacing a single character.
enum Replacement {
    /// Remove the character entirely.
    Empty,
    /// Replace with a single character (includes passthrough).
    Single(char),
    /// Replace with a multi-character string.
    Str(&'static str),
}

/// The main normalizer. Holds a configuration and performs character-level
/// normalization.
#[derive(Debug, Clone)]
pub struct Normalizer {
    config: NormalizerConfig,
}

impl Normalizer {
    /// Create a normalizer with the given config.
    pub fn with_config(config: NormalizerConfig) -> Self {
        Self { config }
    }

    /// Create a normalizer with default settings.
    pub fn new() -> Self {
        Self {
            config: NormalizerConfig::new(),
        }
    }

    /// Normalize the input string according to the configured categories.
    pub fn normalize(&self, input: &str) -> String {
        let cats = self.config.categories;
        let symbols_enabled = cats.contains(Categories::SYMBOLS);
        let whitespace_enabled = cats.contains(Categories::WHITESPACE);

        // Phase 1: character-level replacement
        let mut output = String::with_capacity(input.len());
        let mut prev_was_unicode_bullet = false;
        for ch in input.chars() {
            // Bullet lists copied from rich text often use `<bullet><tab>`.
            // Keep a single separator space after bullet normalization.
            if prev_was_unicode_bullet && whitespace_enabled && ch == '\t' {
                output.push(' ');
                prev_was_unicode_bullet = false;
                continue;
            }

            match self.replace_char(ch, cats) {
                Replacement::Empty => {}
                Replacement::Single(c) => output.push(c),
                Replacement::Str(s) => output.push_str(s),
            }

            prev_was_unicode_bullet = symbols_enabled && Self::is_unicode_bullet(ch);
        }

        output
    }

    #[inline]
    fn is_unicode_bullet(ch: char) -> bool {
        matches!(ch, '\u{2022}' | '\u{25E6}' | '\u{2023}' | '\u{2043}')
    }

    #[inline]
    fn replace_char(&self, ch: char, cats: Categories) -> Replacement {
        match ch {
            // === QUOTES ===
            '\u{2018}' | '\u{2019}' | '\u{201A}' | '\u{2039}' | '\u{203A}'
                if cats.contains(Categories::QUOTES) =>
            {
                Replacement::Single('\'')
            }
            // Left/right double quotes, double low-9 quote, guillemets
            '\u{201C}' | '\u{201D}' | '\u{201E}' | '\u{00AB}' | '\u{00BB}'
                if cats.contains(Categories::QUOTES) =>
            {
                Replacement::Single('"')
            }

            // === DASHES ===
            // En-dash
            '\u{2013}' if cats.contains(Categories::DASHES) => Replacement::Single('-'),
            // Em-dash
            '\u{2014}' if cats.contains(Categories::DASHES) => Replacement::Single('-'),
            // Figure dash
            '\u{2012}' if cats.contains(Categories::DASHES) => Replacement::Single('-'),
            // Horizontal bar
            '\u{2015}' if cats.contains(Categories::DASHES) => Replacement::Single('-'),
            // Minus sign
            '\u{2212}' if cats.contains(Categories::DASHES) => Replacement::Single('-'),
            // Non-breaking hyphen
            '\u{2011}' if cats.contains(Categories::DASHES) => Replacement::Single('-'),
            // Hyphen
            '\u{2010}' if cats.contains(Categories::DASHES) => Replacement::Single('-'),

            // === WHITESPACE ===
            // No-break space
            '\u{00A0}' if cats.contains(Categories::WHITESPACE) => Replacement::Single(' '),
            // Ogham space mark
            '\u{1680}' if cats.contains(Categories::WHITESPACE) => Replacement::Single(' '),
            // En quad
            '\u{2000}' if cats.contains(Categories::WHITESPACE) => Replacement::Single(' '),
            // Em quad
            '\u{2001}' if cats.contains(Categories::WHITESPACE) => Replacement::Single(' '),
            // En space
            '\u{2002}' if cats.contains(Categories::WHITESPACE) => Replacement::Single(' '),
            // Em space
            '\u{2003}' if cats.contains(Categories::WHITESPACE) => Replacement::Single(' '),
            // Three-per-em space
            '\u{2004}' if cats.contains(Categories::WHITESPACE) => Replacement::Single(' '),
            // Four-per-em space
            '\u{2005}' if cats.contains(Categories::WHITESPACE) => Replacement::Single(' '),
            // Six-per-em space
            '\u{2006}' if cats.contains(Categories::WHITESPACE) => Replacement::Single(' '),
            // Figure space
            '\u{2007}' if cats.contains(Categories::WHITESPACE) => Replacement::Single(' '),
            // Punctuation space
            '\u{2008}' if cats.contains(Categories::WHITESPACE) => Replacement::Single(' '),
            // Thin space
            '\u{2009}' if cats.contains(Categories::WHITESPACE) => Replacement::Single(' '),
            // Hair space
            '\u{200A}' if cats.contains(Categories::WHITESPACE) => Replacement::Single(' '),
            // Narrow no-break space
            '\u{202F}' if cats.contains(Categories::WHITESPACE) => Replacement::Single(' '),
            // Medium mathematical space
            '\u{205F}' if cats.contains(Categories::WHITESPACE) => Replacement::Single(' '),
            // Ideographic space
            '\u{3000}' if cats.contains(Categories::WHITESPACE) => Replacement::Single(' '),
            // Tab
            '\t' if cats.contains(Categories::WHITESPACE) => Replacement::Str("  "),

            // === ZERO-WIDTH ===
            // BOM / ZWNBSP
            '\u{FEFF}' if cats.contains(Categories::ZERO_WIDTH) => Replacement::Empty,
            // Zero-width space
            '\u{200B}' if cats.contains(Categories::ZERO_WIDTH) => Replacement::Empty,
            // Zero-width joiner
            '\u{200D}' if cats.contains(Categories::ZERO_WIDTH) => Replacement::Empty,
            // Zero-width non-joiner
            '\u{200C}' if cats.contains(Categories::ZERO_WIDTH) => Replacement::Empty,
            // Left-to-right mark
            '\u{200E}' if cats.contains(Categories::ZERO_WIDTH) => Replacement::Empty,
            // Right-to-left mark
            '\u{200F}' if cats.contains(Categories::ZERO_WIDTH) => Replacement::Empty,
            // Word joiner
            '\u{2060}' if cats.contains(Categories::ZERO_WIDTH) => Replacement::Empty,
            // Left-to-right embedding
            '\u{202A}' if cats.contains(Categories::ZERO_WIDTH) => Replacement::Empty,
            // Right-to-left embedding
            '\u{202B}' if cats.contains(Categories::ZERO_WIDTH) => Replacement::Empty,
            // Pop directional formatting
            '\u{202C}' if cats.contains(Categories::ZERO_WIDTH) => Replacement::Empty,
            // Left-to-right override
            '\u{202D}' if cats.contains(Categories::ZERO_WIDTH) => Replacement::Empty,
            // Right-to-left override
            '\u{202E}' if cats.contains(Categories::ZERO_WIDTH) => Replacement::Empty,
            // Left-to-right isolate
            '\u{2066}' if cats.contains(Categories::ZERO_WIDTH) => Replacement::Empty,
            // Right-to-left isolate
            '\u{2067}' if cats.contains(Categories::ZERO_WIDTH) => Replacement::Empty,
            // First strong isolate
            '\u{2068}' if cats.contains(Categories::ZERO_WIDTH) => Replacement::Empty,
            // Pop directional isolate
            '\u{2069}' if cats.contains(Categories::ZERO_WIDTH) => Replacement::Empty,
            // Arabic letter mark
            '\u{061C}' if cats.contains(Categories::ZERO_WIDTH) => Replacement::Empty,
            // Combining grapheme joiner
            '\u{034F}' if cats.contains(Categories::ZERO_WIDTH) => Replacement::Empty,
            // Soft hyphen
            '\u{00AD}' if cats.contains(Categories::ZERO_WIDTH) => Replacement::Empty,
            // Variation selectors
            '\u{FE00}'..='\u{FE0F}' if cats.contains(Categories::ZERO_WIDTH) => Replacement::Empty,
            '\u{E0100}'..='\u{E01EF}' if cats.contains(Categories::ZERO_WIDTH) => {
                Replacement::Empty
            }

            // === ARROWS ===
            // →
            '\u{2192}' if cats.contains(Categories::ARROWS) => Replacement::Str("->"),
            // ←
            '\u{2190}' if cats.contains(Categories::ARROWS) => Replacement::Str("<-"),
            // ⇒
            '\u{21D2}' if cats.contains(Categories::ARROWS) => Replacement::Str("=>"),
            // ⇐
            '\u{21D0}' if cats.contains(Categories::ARROWS) => Replacement::Str("<="),
            // ↔
            '\u{2194}' if cats.contains(Categories::ARROWS) => Replacement::Str("<->"),
            // ⇔
            '\u{21D4}' if cats.contains(Categories::ARROWS) => Replacement::Str("<=>"),
            // ↑
            '\u{2191}' if cats.contains(Categories::ARROWS) => Replacement::Single('^'),
            // ↓
            '\u{2193}' if cats.contains(Categories::ARROWS) => Replacement::Single('v'),
            // ⟵
            '\u{27F5}' if cats.contains(Categories::ARROWS) => Replacement::Str("<-"),
            // ⟶
            '\u{27F6}' if cats.contains(Categories::ARROWS) => Replacement::Str("->"),
            // ⟷
            '\u{27F7}' if cats.contains(Categories::ARROWS) => Replacement::Str("<->"),

            // === FRACTIONS ===
            // ½
            '\u{00BD}' if cats.contains(Categories::FRACTIONS) => Replacement::Str("1/2"),
            // ¼
            '\u{00BC}' if cats.contains(Categories::FRACTIONS) => Replacement::Str("1/4"),
            // ¾
            '\u{00BE}' if cats.contains(Categories::FRACTIONS) => Replacement::Str("3/4"),
            // ⅓
            '\u{2153}' if cats.contains(Categories::FRACTIONS) => Replacement::Str("1/3"),
            // ⅔
            '\u{2154}' if cats.contains(Categories::FRACTIONS) => Replacement::Str("2/3"),
            // ⅕
            '\u{2155}' if cats.contains(Categories::FRACTIONS) => Replacement::Str("1/5"),
            // ⅖
            '\u{2156}' if cats.contains(Categories::FRACTIONS) => Replacement::Str("2/5"),
            // ⅗
            '\u{2157}' if cats.contains(Categories::FRACTIONS) => Replacement::Str("3/5"),
            // ⅘
            '\u{2158}' if cats.contains(Categories::FRACTIONS) => Replacement::Str("4/5"),
            // ⅙
            '\u{2159}' if cats.contains(Categories::FRACTIONS) => Replacement::Str("1/6"),
            // ⅚
            '\u{215A}' if cats.contains(Categories::FRACTIONS) => Replacement::Str("5/6"),
            // ⅛
            '\u{215B}' if cats.contains(Categories::FRACTIONS) => Replacement::Str("1/8"),
            // ⅜
            '\u{215C}' if cats.contains(Categories::FRACTIONS) => Replacement::Str("3/8"),
            // ⅝
            '\u{215D}' if cats.contains(Categories::FRACTIONS) => Replacement::Str("5/8"),
            // ⅞
            '\u{215E}' if cats.contains(Categories::FRACTIONS) => Replacement::Str("7/8"),

            // === MATH ===
            // × multiplication sign
            '\u{00D7}' if cats.contains(Categories::MATH) => Replacement::Single('x'),
            // ÷ division sign
            '\u{00F7}' if cats.contains(Categories::MATH) => Replacement::Single('/'),
            // ≤ less-than or equal
            '\u{2264}' if cats.contains(Categories::MATH) => Replacement::Str("<="),
            // ≥ greater-than or equal
            '\u{2265}' if cats.contains(Categories::MATH) => Replacement::Str(">="),
            // ≠ not equal
            '\u{2260}' if cats.contains(Categories::MATH) => Replacement::Str("!="),
            // ≈ almost equal
            '\u{2248}' if cats.contains(Categories::MATH) => Replacement::Str("~="),
            // ± plus-minus
            '\u{00B1}' if cats.contains(Categories::MATH) => Replacement::Str("+/-"),
            // ∞ infinity
            '\u{221E}' if cats.contains(Categories::MATH) => Replacement::Str("inf"),
            // √ square root
            '\u{221A}' if cats.contains(Categories::MATH) => Replacement::Str("sqrt"),
            // · middle dot (as multiplication)
            '\u{00B7}' if cats.contains(Categories::MATH) => Replacement::Single('*'),
            // ∑ summation
            '\u{2211}' if cats.contains(Categories::MATH) => Replacement::Str("sum"),
            // ∏ product
            '\u{220F}' if cats.contains(Categories::MATH) => Replacement::Str("prod"),
            // ∆ delta / increment
            '\u{2206}' if cats.contains(Categories::MATH) => Replacement::Str("delta"),

            // === SYMBOLS ===
            // … ellipsis
            '\u{2026}' if cats.contains(Categories::SYMBOLS) => Replacement::Str("..."),
            // • bullet
            '\u{2022}' if cats.contains(Categories::SYMBOLS) => Replacement::Single('-'),
            // ◦ white bullet
            '\u{25E6}' if cats.contains(Categories::SYMBOLS) => Replacement::Single('-'),
            // ‣ triangular bullet
            '\u{2023}' if cats.contains(Categories::SYMBOLS) => Replacement::Single('-'),
            // ⁃ hyphen bullet
            '\u{2043}' if cats.contains(Categories::SYMBOLS) => Replacement::Single('-'),
            // © copyright
            '\u{00A9}' if cats.contains(Categories::SYMBOLS) => Replacement::Str("(c)"),
            // ® registered
            '\u{00AE}' if cats.contains(Categories::SYMBOLS) => Replacement::Str("(R)"),
            // ™ trademark
            '\u{2122}' if cats.contains(Categories::SYMBOLS) => Replacement::Str("(TM)"),
            // ° degree
            '\u{00B0}' if cats.contains(Categories::SYMBOLS) => Replacement::Str(" deg"),
            // ¶ pilcrow
            '\u{00B6}' if cats.contains(Categories::SYMBOLS) => Replacement::Str("[P]"),
            // § section sign
            '\u{00A7}' if cats.contains(Categories::SYMBOLS) => Replacement::Str("[S]"),
            // † dagger
            '\u{2020}' if cats.contains(Categories::SYMBOLS) => Replacement::Single('+'),
            // ‡ double dagger
            '\u{2021}' if cats.contains(Categories::SYMBOLS) => Replacement::Str("++"),
            // ✓ check mark
            '\u{2713}' if cats.contains(Categories::SYMBOLS) => Replacement::Str("[x]"),
            // ✗ ballot x
            '\u{2717}' if cats.contains(Categories::SYMBOLS) => Replacement::Str("[ ]"),

            // === UMLAUTS ===
            'ä' if cats.contains(Categories::UMLAUTS) => Replacement::Str("ae"),
            'Ä' if cats.contains(Categories::UMLAUTS) => Replacement::Str("Ae"),
            'ö' if cats.contains(Categories::UMLAUTS) => Replacement::Str("oe"),
            'Ö' if cats.contains(Categories::UMLAUTS) => Replacement::Str("Oe"),
            'ü' if cats.contains(Categories::UMLAUTS) => Replacement::Str("ue"),
            'Ü' if cats.contains(Categories::UMLAUTS) => Replacement::Str("Ue"),
            'ß' if cats.contains(Categories::UMLAUTS) => Replacement::Str("ss"),

            // === DEFAULT: pass through ===
            _ => Replacement::Single(ch),
        }
    }
}

impl Default for Normalizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quotes_normalized() {
        let n = Normalizer::new();
        assert_eq!(n.normalize("\u{201C}hello\u{201D}"), "\"hello\"");
        assert_eq!(n.normalize("\u{2018}hello\u{2019}"), "'hello'");
    }

    #[test]
    fn dashes_normalized() {
        let n = Normalizer::new();
        assert_eq!(n.normalize("a\u{2014}b"), "a-b"); // em-dash
        assert_eq!(n.normalize("a\u{2013}b"), "a-b"); // en-dash
        assert_eq!(n.normalize("a\u{2010}b"), "a-b"); // hyphen
    }

    #[test]
    fn zero_width_removed() {
        let n = Normalizer::new();
        assert_eq!(n.normalize("a\u{200B}b"), "ab");
        assert_eq!(n.normalize("\u{FEFF}hello"), "hello");
        assert_eq!(n.normalize("a\u{2066}b\u{2069}"), "ab");
        assert_eq!(n.normalize("a\u{00AD}b"), "ab");
        assert_eq!(n.normalize("a\u{061C}b"), "ab");
        assert_eq!(n.normalize("a\u{034F}b"), "ab");
        assert_eq!(n.normalize("\u{2764}\u{FE0F}"), "\u{2764}");
    }

    #[test]
    fn whitespace_normalized() {
        let n = Normalizer::new();
        assert_eq!(n.normalize("a\u{00A0}b"), "a b");
        assert_eq!(n.normalize("a\u{2003}b"), "a b");
        assert_eq!(n.normalize("a\u{1680}b"), "a b");
        assert_eq!(n.normalize("a\u{2000}b"), "a b");
        assert_eq!(n.normalize("a\u{2001}b"), "a b");
    }

    #[test]
    fn arrows_normalized() {
        let n = Normalizer::new();
        assert_eq!(n.normalize("\u{2192}"), "->");
        assert_eq!(n.normalize("\u{2190}"), "<-");
        assert_eq!(n.normalize("\u{21D2}"), "=>");
        assert_eq!(n.normalize("\u{27F6}"), "->");
        assert_eq!(n.normalize("\u{27F5}"), "<-");
    }

    #[test]
    fn fractions_normalized() {
        let n = Normalizer::new();
        assert_eq!(n.normalize("\u{00BD}"), "1/2");
        assert_eq!(n.normalize("\u{00BC}"), "1/4");
        assert_eq!(n.normalize("\u{00BE}"), "3/4");
    }

    #[test]
    fn math_normalized() {
        let n = Normalizer::new();
        assert_eq!(n.normalize("\u{00D7}"), "x");
        assert_eq!(n.normalize("\u{00F7}"), "/");
        assert_eq!(n.normalize("\u{2264}"), "<=");
        assert_eq!(n.normalize("\u{2260}"), "!=");
    }

    #[test]
    fn symbols_normalized() {
        let n = Normalizer::new();
        assert_eq!(n.normalize("\u{2026}"), "...");
        assert_eq!(n.normalize("\u{2022}"), "-");
        assert_eq!(n.normalize("\u{00A9}"), "(c)");
        assert_eq!(n.normalize("\u{00AE}"), "(R)");
        assert_eq!(n.normalize("\u{2122}"), "(TM)");
    }

    #[test]
    fn bullet_with_tab_separator_normalized_to_single_space() {
        let n = Normalizer::new();
        assert_eq!(n.normalize("\u{2022}\tM\u{00F6}we"), "- M\u{00F6}we");
    }

    #[test]
    fn umlauts_off_by_default() {
        let n = Normalizer::new();
        assert_eq!(n.normalize("ä"), "ä");
        assert_eq!(n.normalize("ö"), "ö");
    }

    #[test]
    fn umlauts_when_enabled() {
        let n = Normalizer::with_config(NormalizerConfig::new().umlauts(true));
        assert_eq!(n.normalize("ä"), "ae");
        assert_eq!(n.normalize("Ö"), "Oe");
        assert_eq!(n.normalize("ß"), "ss");
    }

    #[test]
    fn category_disabled() {
        let n = Normalizer::with_config(NormalizerConfig::new().quotes(false));
        assert_eq!(
            n.normalize("\u{201C}hello\u{201D}"),
            "\u{201C}hello\u{201D}"
        );
    }

    #[test]
    fn ascii_passthrough() {
        let n = Normalizer::new();
        let ascii = "Hello, world! 123 foo-bar_baz";
        assert_eq!(n.normalize(ascii), ascii);
    }
}
