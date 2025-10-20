# Oddsidizer

Rust library with tools for converting and formatting data around betting:

1. odds: american, decimal and fractional, formats producing pretty and traditional fractions for display,
2. race distance (miles, furlongs, yards).

## Features

* Based on `rust_decimal`
* Uses lookup tables
* Performs fractional rounding

## IMPORTANT NOTES

* Keep in mind that when using lookup tables feature:
  * conversion from 1.67 or -150 gives 4/6 instead of 2/3
  * conversion from 2.5 or 150 gives 6/4 instead of 3/2

  These are traditional values in UK.

* Disabling fractional rounding it but leaving lookup tables enabled can still return pre-defined rounded results (f. ex. 1.33 -> 1/3).
