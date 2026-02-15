//! Energy-related calculators including
//! power consumption and electricity cost.

pub mod electricity_cost;
pub mod power_consumption;

pub use electricity_cost::electricity_cost;
pub use power_consumption::energy_consumption;
