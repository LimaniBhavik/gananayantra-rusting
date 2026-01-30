//! Specialized industry calculators.

pub mod building;
pub mod date_time;
pub mod electronics;
pub mod network_cctv;
pub mod science;

pub use building::calculate_square_footage;
