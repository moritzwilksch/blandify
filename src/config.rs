use crate::categories::Categories;

/// Configuration for the normalizer, built via a builder pattern.
#[derive(Debug, Clone)]
pub struct NormalizerConfig {
    pub(crate) categories: Categories,
}

impl NormalizerConfig {
    /// Create a new config with default categories (all except UMLAUTS).
    pub fn new() -> Self {
        Self {
            categories: Categories::DEFAULT,
        }
    }

    /// Create a config from an explicit set of categories.
    pub fn from_categories(categories: Categories) -> Self {
        Self { categories }
    }

    /// Get the configured categories.
    pub fn categories(&self) -> Categories {
        self.categories
    }

    /// Enable or disable smart quote normalization.
    pub fn quotes(mut self, enabled: bool) -> Self {
        self.categories.set(Categories::QUOTES, enabled);
        self
    }

    /// Enable or disable dash normalization.
    pub fn dashes(mut self, enabled: bool) -> Self {
        self.categories.set(Categories::DASHES, enabled);
        self
    }

    /// Enable or disable Unicode whitespace normalization.
    pub fn whitespace(mut self, enabled: bool) -> Self {
        self.categories.set(Categories::WHITESPACE, enabled);
        self
    }

    /// Enable or disable zero-width character removal.
    pub fn zero_width(mut self, enabled: bool) -> Self {
        self.categories.set(Categories::ZERO_WIDTH, enabled);
        self
    }

    /// Enable or disable arrow normalization.
    pub fn arrows(mut self, enabled: bool) -> Self {
        self.categories.set(Categories::ARROWS, enabled);
        self
    }

    /// Enable or disable fraction normalization.
    pub fn fractions(mut self, enabled: bool) -> Self {
        self.categories.set(Categories::FRACTIONS, enabled);
        self
    }

    /// Enable or disable math operator normalization.
    pub fn math(mut self, enabled: bool) -> Self {
        self.categories.set(Categories::MATH, enabled);
        self
    }

    /// Enable or disable symbol normalization.
    pub fn symbols(mut self, enabled: bool) -> Self {
        self.categories.set(Categories::SYMBOLS, enabled);
        self
    }

    /// Enable or disable German umlaut expansion.
    pub fn umlauts(mut self, enabled: bool) -> Self {
        self.categories.set(Categories::UMLAUTS, enabled);
        self
    }

    /// Enable or disable control character stripping.
    pub fn control_chars(mut self, enabled: bool) -> Self {
        self.categories.set(Categories::CONTROL_CHARS, enabled);
        self
    }

    /// Build a `Normalizer` from this config.
    pub fn build(self) -> crate::normalizer::Normalizer {
        crate::normalizer::Normalizer::with_config(self)
    }
}

impl Default for NormalizerConfig {
    fn default() -> Self {
        Self::new()
    }
}
