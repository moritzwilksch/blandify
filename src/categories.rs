use bitflags::bitflags;

bitflags! {
    /// Categories of Unicode normalization that can be independently toggled.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Categories: u16 {
        /// Smart/curly quotes, guillemets → ASCII quotes
        const QUOTES      = 0x0001;
        /// En-dash, em-dash, figure dash, horizontal bar, minus → `-`
        const DASHES      = 0x0002;
        /// NBSP, en/em space, ogham/quad spaces, thin space, hair space, etc. → ASCII space
        const WHITESPACE  = 0x0004;
        /// BOM, ZWSP/ZWJ/ZWNJ, directional marks, soft hyphen, variation selectors → removed
        const ZERO_WIDTH  = 0x0008;
        /// `→` → `->`, `←` → `<-`, `⇒` → `=>`, etc.
        const ARROWS      = 0x0010;
        /// `½` → `1/2`, `¼` → `1/4`, etc.
        const FRACTIONS   = 0x0020;
        /// `×` → `x`, `÷` → `/`, `≤` → `<=`, `≠` → `!=`, etc.
        const MATH        = 0x0040;
        /// Bullets → `-`, `…` → `...`, `©` → `(c)`, etc.
        const SYMBOLS     = 0x0080;
        /// `ä` → `ae`, `ö` → `oe`, `ü` → `ue`, `ß` → `ss` (lossy, language-specific)
        const UMLAUTS     = 0x0100;

        /// All categories except UMLAUTS (default configuration)
        const DEFAULT = Self::QUOTES.bits()
            | Self::DASHES.bits()
            | Self::WHITESPACE.bits()
            | Self::ZERO_WIDTH.bits()
            | Self::ARROWS.bits()
            | Self::FRACTIONS.bits()
            | Self::MATH.bits()
            | Self::SYMBOLS.bits();

        /// All categories including UMLAUTS
        const ALL = Self::DEFAULT.bits() | Self::UMLAUTS.bits();
    }
}
