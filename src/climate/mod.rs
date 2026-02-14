//! Climate and environmental calculators.

pub mod carbon;
pub mod energy;

pub use carbon::{calculate_carbon_footprint_electricity, calculate_carbon_footprint_fuel};
pub use energy::{calculate_solar_roi, calculate_battery_capacity};
