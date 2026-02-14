//! Water-related calculators including
//! pressure at depth and river flow rate.

pub mod pressure_at_depth;
pub mod river_flow_rate;

pub use pressure_at_depth::pressure_at_depth;
pub use river_flow_rate::river_flow_rate;
