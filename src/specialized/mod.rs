//! Specialized industry calculators.

pub mod date_time;
pub mod building;
pub mod science;
pub mod electronics;
pub mod network_cctv;

pub use building::calculate_square_footage;
