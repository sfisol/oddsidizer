use rust_decimal::RoundingStrategy;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FractionStrategy {
    /// Plain method (less precise, but faster, f. ex. 1.33 gives 33/100 instead of 1/3).
    ///
    /// Note that using None method but leaving lookup enabled can still return simplified fractions (f. ex. 1.33 -> 1/3) from the lookup tables (see README.md)
    Plain,
    /// Use a continued fraction algorithm for better precision and simple fractions. (1.33 gives 1/3 instead of 33/100)
    Simplify,
}

/// Configuration for conversion functions.
#[derive(Debug, Clone, Copy)]
pub struct ConversionConfig {
    /// Use lookup tables first for conversion, then fallback to regular computations
    /// Note: When using lookup tables feature, conversion from 1.67 or -150 gives 4/6 instead of 2/3 (see README.md)
    pub use_lookup_tables: bool,
    /// Fractions computing strategy
    pub fraction_strategy: FractionStrategy,
    /// Rounding method for Decimal type
    pub rounding_strategy: RoundingStrategy,
}

impl Default for ConversionConfig {
    /// Provides standard settings.
    ///
    /// - lookup enabled
    /// - fractions simplified
    /// - MidpointAwayFromZero (RoundHalfUp) rounding strategy
    fn default() -> Self {
        DEFAULT_CONVERSION_CONFIG
    }
}

static DEFAULT_CONVERSION_CONFIG: ConversionConfig = ConversionConfig {
    use_lookup_tables: true,
    fraction_strategy: FractionStrategy::Simplify,
    rounding_strategy: RoundingStrategy::MidpointAwayFromZero, // former RoundHalfUp
};

impl ConversionConfig {
    pub fn no_lookup(&mut self) -> &mut Self {
        self.use_lookup_tables = false;
        self
    }

    pub fn plain_fraction_strategy(&mut self) -> &mut Self {
        self.fraction_strategy = FractionStrategy::Plain;
        self
    }

    pub fn fraction_strategy(&mut self, strategy: FractionStrategy) -> &mut Self {
        self.fraction_strategy = strategy;
        self
    }

    pub fn rounding_strategy(&mut self, strategy: RoundingStrategy) -> &mut Self {
        self.rounding_strategy = strategy;
        self
    }
}
