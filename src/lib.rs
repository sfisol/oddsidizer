#![doc = pretty_readme::docify!("README.md", "https://docs.rs/oddsidizer/latest/oddsidizer/", "./")]

mod config;
pub use config::*;

mod convert;
pub use convert::*;

mod distance;
pub use distance::RaceDistance;

mod lookup_tables;

mod lookup_funcs;
pub use lookup_funcs::*;

mod odds;
pub use odds::*;

#[cfg(test)]
mod testing_helpers;
