use rust_decimal::Decimal;

use crate::{
    ConversionConfig, LookupVariant,
    lookup_tables::{
        get_american_to_fraction_extended_map, get_american_to_fraction_map,
        get_decimal_to_fraction_extended_map, get_decimal_to_fraction_map,
    },
};

/// Manually Lookup decimal to fractional table using provided config
pub fn lookup_decimal_to_fraction_with_config(
    odds: Decimal,
    config: ConversionConfig,
) -> Option<(u32, u32)> {
    let frac = get_decimal_to_fraction_map().get(&odds);

    if frac.is_none() && config.lookup_tables_variant == LookupVariant::Extended {
        return get_decimal_to_fraction_extended_map().get(&odds).copied();
    }

    frac.copied()
}

/// Manually Lookup decimal to fractional table using default config (no extended tables used)
pub fn lookup_decimal_to_fraction(odds: Decimal) -> Option<(u32, u32)> {
    lookup_decimal_to_fraction_with_config(odds, ConversionConfig::default())
}

/// Manually Lookup american to fractional table using provided config
pub fn lookup_american_to_fraction_with_config(
    odds: i32,
    config: ConversionConfig,
) -> Option<(u32, u32)> {
    let frac = get_american_to_fraction_map().get(&odds);

    if frac.is_none() && config.lookup_tables_variant == LookupVariant::Extended {
        return get_american_to_fraction_extended_map().get(&odds).copied();
    }

    frac.copied()
}

/// Manually Lookup american to fractional table using default config (no extended tables used)
pub fn lookup_american_to_fraction(odds: i32) -> Option<(u32, u32)> {
    lookup_american_to_fraction_with_config(odds, ConversionConfig::default())
}
