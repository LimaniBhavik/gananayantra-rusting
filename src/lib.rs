//! # gananayantra-rusting
//!
//! **gananayantra-rusting** is a multi-industry calculation engine written in Rust.
//! It provides reusable, library-first calculators across domains such as:
//!
//! - Finance (ROI, TVM, compound interest)
//! - Energy (power consumption, electricity cost)
//! - Geo (Earth distance, horizon distance)
//! - Water (pressure at depth, river flow rate)
//! - Space (orbital velocity, escape velocity, orbital period)
//! - Utilities (percentage calculations)
//!
//! ## Example
//! ```rust
//! use gananayantra::finance::tvm::future_value;
//!
//! let fv = future_value(10_000.0, 0.08, 5).unwrap();
//! assert!(fv > 14_000.0);
//! ```
//!
//! This crate is **library-first** and does not impose any CLI or I/O model.

pub mod energy;
pub mod finance;
pub mod geo;
pub mod health;
pub mod space;
pub mod utilities;
pub mod water;

// Internal modules (not exposed to library users yet)
mod auto;
mod specialized;
mod utility_lifestyle;
mod calculators;
mod math;
mod finance_investment;
