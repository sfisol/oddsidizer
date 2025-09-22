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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LookupVariant {
    /// No lookup
    None,
    /// Basic lookup for common values usually matching closely the original value
    Basic,
    /// Extended lookup - covers more values but also gives more rounded results, f. ex. 1.0013 -> 1/750 instead of 1/768
    Extended,
}

/// Configuration for conversion functions.
#[derive(Debug, Clone, Copy)]
pub struct ConversionConfig {
    /// Use lookup tables first for conversion, then fallback to regular computations
    /// Note: When using lookup tables feature, conversion from 1.67 or -150 gives 4/6 instead of 2/3 (see README.md)
    pub lookup_tables_variant: LookupVariant,
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
    lookup_tables_variant: LookupVariant::Basic,
    fraction_strategy: FractionStrategy::Simplify,
    rounding_strategy: RoundingStrategy::MidpointAwayFromZero, // former RoundHalfUp
};

impl ConversionConfig {
    pub fn no_lookup(mut self) -> Self {
        self.lookup_tables_variant = LookupVariant::None;
        self
    }

    pub fn extended_lookup(mut self) -> Self {
        self.lookup_tables_variant = LookupVariant::Extended;
        self
    }

    pub fn plain_fraction_strategy(mut self) -> Self {
        self.fraction_strategy = FractionStrategy::Plain;
        self
    }

    pub fn fraction_strategy(mut self, strategy: FractionStrategy) -> Self {
        self.fraction_strategy = strategy;
        self
    }

    pub fn rounding_strategy(mut self, strategy: RoundingStrategy) -> Self {
        self.rounding_strategy = strategy;
        self
    }
}
