//! Climate and environmental calculators.

pub mod carbon;
pub mod energy;
pub mod mobility;
pub mod water;

pub use carbon::{calculate_carbon_footprint_electricity, calculate_carbon_footprint_fuel};
pub use energy::{calculate_solar_roi, calculate_battery_capacity};
pub use mobility::{calculate_ev_charging_cost, calculate_ev_range};
pub use water::calculate_industrial_water_footprint;
