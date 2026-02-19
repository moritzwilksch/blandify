pub mod categories;
pub mod config;
pub mod normalizer;

pub use categories::Categories;
pub use config::NormalizerConfig;
pub use normalizer::Normalizer;

/// Normalize text using default settings (all categories except UMLAUTS).
pub fn normalize(input: &str) -> String {
    Normalizer::new().normalize(input)
}
