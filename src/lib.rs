//! gananayantra-rusting
//!
//! A multi-industry calculation engine covering finance, energy,
//! geospatial, water, space, and utility domains.

pub mod energy;
pub mod finance;
pub mod geo;
pub mod space;
pub mod utilities;
pub mod water;

// Internal modules (not exposed to library users yet)
mod auto;
mod health;
mod specialized;
mod utility_lifestyle;
mod calculators;
mod math;
mod finance_investment;
