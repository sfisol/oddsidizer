# Oddsidizer

A rust library with tools for converting and formatting data around betting:

1. odds: american, decimal and fractional, formats producing pretty and traditional fractions for display,
2. race distance (miles, furlongs, yards).

## Features

* Based on `rust_decimal`
* Uses lookup tables
* Performs fractional rounding

### IMPORTANT NOTES

* Keep in mind that when using lookup tables feature:
  * conversion from 1.67 or -150 gives 4/6 instead of 2/3
  * conversion from 2.5 or 150 gives 6/4 instead of 3/2

  These are traditional values in the UK.

* Disabling fractional rounding but leaving lookup tables enabled can still return pre-defined rounded results (f. ex. 1.33 -> 1/3).

## Usage

### `Odds`

The [`Odds`] enum is a convenient wrapper that holds an odds value in any of the three formats. It provides methods to easily convert to any other format.

```rust
use rust_decimal_macros::dec;
use oddsidizer::{Odds, ConversionConfig};

// --- Creating Odds ---
// Use `From` or `Into` to create instances
let american_odds = Odds::from(-150);
let decimal_odds: Odds = dec!(1.25).into();
let fractional_odds = Odds::from((1, 4));

// --- Easy Conversions ---
// Convert to any format using default config
assert_eq!(american_odds.to_decimal().unwrap(), dec!(1.67)); // Uses lookup
assert_eq!(decimal_odds.to_american().unwrap(), -400);
assert_eq!(fractional_odds.to_decimal().unwrap(), dec!(1.25));

// Passthrough: converting to its own type is a no-op
assert_eq!(american_odds.to_american().unwrap(), -150);

// --- String Formatting ---
// `to_fractional_str` converts and formats
assert_eq!(american_odds.to_fractional_str().unwrap(), "4/6");
assert_eq!(decimal_odds.to_fractional_str().unwrap(), "1/4");

// `to_decimal_str` converts and formats to 2 decimal places
let fractional_odds_2 = Odds::from((2, 3)); // 1.666...
assert_eq!(fractional_odds_2.to_decimal_str().unwrap(), "1.67");
assert_eq!(american_odds.to_decimal_str().unwrap(), "1.67");

// --- Custom Conversions ---
// Use the `_custom` methods to pass a config
let config = ConversionConfig::default().no_lookup();

// Default: -150 (American) -> 4/6 (Fractional) via lookup
assert_eq!(american_odds.to_fractional_str().unwrap(), "4/6");

// Custom: -150 (American) -> 2/3 (Fractional) via calculation
assert_eq!(american_odds.to_fractional_str_custom(&config).unwrap(), "2/3");
```

### `ConversionConfig`

The [`ConversionConfig`] struct allows you to customize conversion behavior, such as:

* which lookup tables to use,
* how to calculate fractions,
* which rounding strategy to apply.

You can create a default config or use the builder pattern to customize it.

```rust
use rust_decimal::RoundingStrategy;
use oddsidizer::{ConversionConfig, FractionStrategy, LookupVariant};

// --- Default Configuration ---
// Uses `LookupVariant::Basic` and `FractionStrategy::Simplify`
let config = ConversionConfig::default();

assert_eq!(config.lookup_tables_variant, LookupVariant::Basic);
assert_eq!(config.fraction_strategy, FractionStrategy::Simplify);
assert_eq!(config.rounding_strategy, RoundingStrategy::MidpointAwayFromZero);

// --- Custom Configuration ---

// Create a config with no lookup tables
let no_lookup_config = ConversionConfig::default().no_lookup();
assert_eq!(no_lookup_config.lookup_tables_variant, LookupVariant::None);

// Create a config with extended (less precise) lookup tables
let extended_config = ConversionConfig::default().extended_lookup();
assert_eq!(extended_config.lookup_tables_variant, LookupVariant::Extended);

// Create a config that uses plain, unsimplified fractions
let plain_config = ConversionConfig::default().plain_fraction_strategy();
assert_eq!(plain_config.fraction_strategy, FractionStrategy::Plain);

// --- Chaining Methods ---
// You can chain methods to build a specific config
let custom_config = ConversionConfig::default()
    .extended_lookup()
    .plain_fraction_strategy()
    .rounding_strategy(RoundingStrategy::RoundDown);

assert_eq!(custom_config.lookup_tables_variant, LookupVariant::Extended);
assert_eq!(custom_config.fraction_strategy, FractionStrategy::Plain);
assert_eq!(custom_config.rounding_strategy, RoundingStrategy::RoundDown);
```

### Core Conversion Functions

These functions provide direct conversions between different odds formats. They all have a default version and a `_custom` version that accepts a [`ConversionConfig`].

```rust
use rust_decimal_macros::dec;
use oddsidizer::{
    american_to_decimal, american_to_fractional, american_to_fractional_custom,
    decimal_to_american, decimal_to_fractional, fractional_to_american,
    fractional_to_decimal, ConversionConfig,
};

// --- American <=> Decimal ---
assert_eq!(american_to_decimal(-110).unwrap(), dec!(1.91)); // From basic lookup
assert_eq!(american_to_decimal(250).unwrap(), dec!(3.5));
assert_eq!(decimal_to_american(dec!(1.5)).unwrap(), -200);
assert_eq!(decimal_to_american(dec!(3.5)).unwrap(), 250);

// --- Fractional <=> Decimal ---
assert_eq!(fractional_to_decimal(1, 2).unwrap(), dec!(1.5));
assert_eq!(fractional_to_decimal(5, 2).unwrap(), dec!(3.5));
assert_eq!(decimal_to_fractional(dec!(1.5)).unwrap(), (1, 2));
assert_eq!(decimal_to_fractional(dec!(1.333)).unwrap(), (1, 3)); // Uses Simplify strategy

// --- American <=> Fractional ---
assert_eq!(american_to_fractional(-200).unwrap(), (1, 2));
assert_eq!(american_to_fractional(250).unwrap(), (5, 2));
assert_eq!(fractional_to_american(1, 2).unwrap(), -200);
assert_eq!(fractional_to_american(5, 2).unwrap(), 250);

// --- Using a Custom Config ---
// The default lookup for -150 gives the common UK fraction 4/6
assert_eq!(american_to_fractional(-150).unwrap(), (4, 6));

// If we disable lookup tables, the function calculates and simplifies the result
let config = ConversionConfig::default().no_lookup();
assert_eq!(american_to_fractional_custom(-150, &config).unwrap(), (2, 3));
```

### Manual Lookup Functions

These functions *only* check the lookup tables and return an `Option`. They do not perform any calculations. This is useful if you *only* want to convert values that have a common, predefined fractional representation.

```rust
use rust_decimal_macros::dec;
use oddsidizer::{
    lookup_american_to_fraction, lookup_decimal_to_fraction,
    lookup_decimal_to_fraction_with_config, ConversionConfig,
};

// --- Basic Lookup ---

// Look up a common value that exists in the basic table
let frac = lookup_american_to_fraction(-150);
assert_eq!(frac, Some((4, 6)));

// Look up a common decimal value
let frac_dec = lookup_decimal_to_fraction(dec!(1.25));
assert_eq!(frac_dec, Some((1, 4)));

// --- Handling Misses ---

// This value doesn't exist in the basic table, so it returns None
let frac_miss = lookup_american_to_fraction(-151);
assert_eq!(frac_miss, None);

// --- Using Extended Tables ---

let extended_config = ConversionConfig::default().extended_lookup();
let dec_val = dec!(1.0013);

// This value is not in the basic table
assert_eq!(lookup_decimal_to_fraction(dec_val), None);

// But it IS in the extended table
let frac_extended = lookup_decimal_to_fraction_with_config(dec_val, extended_config);
assert_eq!(frac_extended, Some((1, 750)));
```

### `RaceDistance`

The [`RaceDistance`] struct represents a distance in miles, furlongs, and yards. It can be constructed from a total yardage and implements `Display` for easy formatting.

```rust
use oddsidizer::RaceDistance;

// --- Create from total yards ---
let dist_long = RaceDistance::from_yards(2000);

assert_eq!(dist_long.miles, 1);
assert_eq!(dist_long.furlongs, 1);
assert_eq!(dist_long.yards, 20);

// --- Display Formatting (via .to_string() or println!) ---
let dist_mile = RaceDistance::from_yards(1760);
let dist_sprint = RaceDistance::from_yards(1320);
let dist_odd = RaceDistance::from_yards(219);
let dist_zero = RaceDistance::from_yards(0);

// Only non-zero parts are included
assert_eq!(dist_long.to_string(), "1m 1f 20y");
assert_eq!(dist_mile.to_string(), "1m");
assert_eq!(dist_sprint.to_string(), "6f");
assert_eq!(dist_odd.to_string(), "219y");
assert_eq!(dist_zero.to_string(), "0y");

// You can print it directly
println!("The race distance is: {}", dist_long);
// Output: The race distance is: 1m 1f 20y
```
