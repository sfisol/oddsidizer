use rust_decimal::Decimal;

use crate::{
    ConversionConfig, ConversionError, american_to_decimal_custom, american_to_fractional_custom,
    decimal_to_american_custom, decimal_to_fractional_custom, fractional_to_american_custom,
    fractional_to_decimal,
};

#[derive(Clone, Copy)]
pub enum Odds {
    American(i32),
    Decimal(Decimal),
    Fractional { num: u32, den: u32 },
}

impl From<i32> for Odds {
    fn from(value: i32) -> Self {
        Self::American(value)
    }
}

impl From<Decimal> for Odds {
    fn from(value: Decimal) -> Self {
        Self::Decimal(value)
    }
}

impl From<(u32, u32)> for Odds {
    fn from((num, den): (u32, u32)) -> Self {
        Self::Fractional { num, den }
    }
}

impl Odds {
    /// Convert from decimal or fractional to american using default parameters. If already american, just return the value.
    pub fn to_american(&self) -> Result<i32, ConversionError> {
        self.to_american_custom(&ConversionConfig::default())
    }

    /// Convert from decimal or fractional to american using custom parameters. If already american, just return the value.
    pub fn to_american_custom(&self, config: &ConversionConfig) -> Result<i32, ConversionError> {
        match self {
            Odds::American(inner) => Ok(*inner),
            Odds::Decimal(decimal) => decimal_to_american_custom(*decimal, config),
            Odds::Fractional { num, den } => fractional_to_american_custom(*num, *den, config),
        }
    }

    /// Convert from american or decimal to fractional using default parameters. If already fractional, just return the value.
    pub fn to_fractional(&self) -> Result<(u32, u32), ConversionError> {
        self.to_fractional_custom(&ConversionConfig::default())
    }

    /// Convert from american or decimal to fractional using custom parameters. If already fractional, just return the value.
    pub fn to_fractional_custom(
        &self,
        config: &ConversionConfig,
    ) -> Result<(u32, u32), ConversionError> {
        match self {
            Odds::American(inner) => american_to_fractional_custom(*inner, config),
            Odds::Decimal(decimal) => decimal_to_fractional_custom(*decimal, config),
            Odds::Fractional { num, den } => {
                if *den > 0 {
                    Ok((*num, *den))
                } else {
                    Err(ConversionError::DenominatorZero)
                }
            }
        }
    }

    /// Convert from american or fractional to decimal using default parameters. If already decimal, just return the value.
    pub fn to_decimal(&self) -> Result<Decimal, ConversionError> {
        self.to_decimal_custom(&ConversionConfig::default())
    }

    /// Convert from american or fractional to decimal using custom parameters. If already decimal, just return the value.
    pub fn to_decimal_custom(&self, config: &ConversionConfig) -> Result<Decimal, ConversionError> {
        match self {
            Odds::American(inner) => american_to_decimal_custom(*inner, config),
            Odds::Decimal(decimal) => {
                if *decimal > Decimal::ONE {
                    Ok(*decimal)
                } else {
                    Err(ConversionError::InvalidDecimal)
                }
            }
            Odds::Fractional { num, den } => fractional_to_decimal(*num, *den),
        }
    }

    /// Convert from american or decimal to fractional using default parameters
    /// (if already fractional, just take the value) and format to string.
    pub fn to_fractional_str(&self) -> Result<String, ConversionError> {
        self.to_fractional_str_custom(&ConversionConfig::default())
    }

    /// Convert from american or decimal to fractional using custom parameters
    /// (if already fractional, just take the value) and format to string.
    pub fn to_fractional_str_custom(
        &self,
        config: &ConversionConfig,
    ) -> Result<String, ConversionError> {
        let (num, den) = self.to_fractional_custom(config)?;
        Ok(format!("{num}/{den}"))
    }

    /// Convert from american or fractional to decimal using default parameters
    /// (if already decimal, just take the value) and format to string.
    pub fn to_decimal_str(&self) -> Result<String, ConversionError> {
        self.to_decimal_str_custom(&ConversionConfig::default())
    }

    /// Convert from american or fractional to decimal using custom parameters
    /// (if already decimal, just take the value) and format to string.
    pub fn to_decimal_str_custom(
        &self,
        config: &ConversionConfig,
    ) -> Result<String, ConversionError> {
        let decimal = self.to_decimal_custom(config)?;
        Ok(format!(
            "{:.2}",
            decimal.round_dp_with_strategy(2, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::testing_helpers::assert_decimal_ok_eq;

    use super::*;
    use rust_decimal_macros::dec;

    // --- Tests for From/Into Implementations ---

    #[test]
    fn test_from_primitives_to_odds() {
        // From<i32>
        if let Odds::American(val) = Odds::from(150) {
            assert_eq!(val, 150);
        } else {
            panic!("Expected Odds::American");
        }

        // From<Decimal>
        if let Odds::Decimal(val) = Odds::from(dec!(3.33)) {
            assert_eq!(val, dec!(3.33));
        } else {
            panic!("Expected Odds::Decimal");
        }

        // From<(u32, u32)>
        if let Odds::Fractional { num, den } = Odds::from((7, 2)) {
            assert_eq!(num, 7);
            assert_eq!(den, 2);
        } else {
            panic!("Expected Odds::Fractional");
        }
    }

    #[test]
    fn test_from_odds_to_i32() {
        let american = Odds::American(-110);
        let decimal = Odds::Decimal(dec!(3.5));
        let fractional = Odds::Fractional { num: 9, den: 1 };
        let invalid_decimal = Odds::Decimal(dec!(1.0));

        assert_eq!(american.to_american(), Ok(-110));
        assert_eq!(decimal.to_american(), Ok(250));
        assert_eq!(fractional.to_american(), Ok(900));
        assert_eq!(
            invalid_decimal.to_american(),
            Err(ConversionError::InvalidDecimal)
        );
    }

    #[test]
    fn test_from_odds_to_fractional_tuple() {
        let american = Odds::American(-150);
        let decimal = Odds::Decimal(dec!(1.25));
        let fractional = Odds::Fractional { num: 9, den: 1 };
        let invalid_american = Odds::American(0);

        assert_eq!(american.to_fractional(), Ok((4, 6)));

        assert_eq!(
            american.to_fractional_custom(&ConversionConfig {
                use_lookup_tables: false,
                ..Default::default()
            }),
            Ok((2, 3))
        );

        assert_eq!(decimal.to_fractional(), Ok((1, 4)));
        assert_eq!(fractional.to_fractional(), Ok((9, 1)));

        assert_eq!(
            invalid_american.to_fractional(),
            Err(ConversionError::AmericanZero)
        );
    }

    #[test]
    fn test_from_odds_to_decimal() {
        let american = Odds::American(200);
        let decimal = Odds::Decimal(dec!(1.75));
        let fractional = Odds::Fractional { num: 1, den: 2 };
        let invalid_american = Odds::American(0);

        assert_decimal_ok_eq(american.to_decimal(), dec!(3.0));
        assert_decimal_ok_eq(decimal.to_decimal(), dec!(1.75));
        assert_decimal_ok_eq(fractional.to_decimal(), dec!(1.5));

        assert_eq!(
            invalid_american.to_decimal(),
            Err(ConversionError::AmericanZero)
        );
    }

    #[test]
    fn test_to_fractional_str() {
        // From American
        assert_eq!(Odds::American(-200).to_fractional_str().unwrap(), "1/2");
        assert_eq!(Odds::American(250).to_fractional_str().unwrap(), "5/2");

        assert_eq!(Odds::American(-150).to_fractional_str().unwrap(), "4/6");

        assert_eq!(
            Odds::American(-150)
                .to_fractional_str_custom(&ConversionConfig {
                    use_lookup_tables: false,
                    ..Default::default()
                })
                .unwrap(),
            "2/3"
        );

        // From Decimal
        assert_eq!(Odds::Decimal(dec!(1.5)).to_fractional_str().unwrap(), "1/2");
        assert_eq!(Odds::Decimal(dec!(3.5)).to_fractional_str().unwrap(), "5/2");
        assert_eq!(
            Odds::Decimal(dec!(2.25)).to_fractional_str().unwrap(),
            "5/4"
        );

        // From Fractional (passthrough)
        assert_eq!(
            Odds::Fractional { num: 7, den: 2 }
                .to_fractional_str()
                .unwrap(),
            "7/2"
        );
        assert_eq!(
            Odds::Fractional { num: 1, den: 1 }
                .to_fractional_str()
                .unwrap(),
            "1/1"
        );

        // Error cases
        assert!(Odds::American(0).to_fractional_str().is_err());
        assert!(Odds::Decimal(dec!(0.9)).to_fractional_str().is_err());
        assert!(
            Odds::Fractional { num: 1, den: 0 }
                .to_fractional_str()
                .is_err()
        );
    }

    #[test]
    fn test_to_decimal_str() {
        // From American
        assert_eq!(Odds::American(-500).to_decimal_str().unwrap(), "1.20");
        assert_eq!(Odds::American(200).to_decimal_str().unwrap(), "3.00");
        assert_eq!(Odds::American(-110).to_decimal_str().unwrap(), "1.91"); // Tests rounding

        // From Decimal
        assert_eq!(Odds::Decimal(dec!(4.0)).to_decimal_str().unwrap(), "4.00");
        assert_eq!(Odds::Decimal(dec!(2.75)).to_decimal_str().unwrap(), "2.75");
        assert_eq!(
            Odds::Decimal(dec!(1.3333)).to_decimal_str().unwrap(),
            "1.33"
        ); // Tests rounding

        // From Fractional
        assert_eq!(
            Odds::Fractional { num: 1, den: 2 }
                .to_decimal_str()
                .unwrap(),
            "1.50"
        );
        assert_eq!(
            Odds::Fractional { num: 4, den: 1 }
                .to_decimal_str()
                .unwrap(),
            "5.00"
        );
        assert_eq!(
            Odds::Fractional { num: 2, den: 3 }
                .to_decimal_str()
                .unwrap(),
            "1.67"
        ); // Tests rounding

        // Error cases
        assert!(Odds::American(0).to_decimal_str().is_err());
        assert!(Odds::Decimal(dec!(-2.0)).to_decimal_str().is_err());
        assert!(
            Odds::Fractional { num: 1, den: 0 }
                .to_decimal_str()
                .is_err()
        );
    }
}
