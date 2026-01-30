//! Climate and environmental calculators.

pub mod carbon;

pub use carbon::{calculate_carbon_footprint_electricity, calculate_carbon_footprint_fuel};
