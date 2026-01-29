//! # gananayantra
//!
//! **gananayantra** is a professional-grade, multi-industry calculation engine written in Rust.
//! It provides reusable, library-first calculators across diverse domains including:
//!
//! - **Finance**: ROI, time value of money, compound interest
//! - **Health**: BMI, BMR, body fat percentage, calorie requirements
//! - **Energy**: Power consumption, electricity cost
//! - **Geo**: Earth distance (Haversine), horizon distance
//! - **Water**: Pressure at depth, river flow rate
//! - **Space**: Orbital velocity, escape velocity, orbital period
//! - **Utilities**: Percentage calculations
//!
//! ## Design Principles
//!
//! This crate is **library-first** and does not impose any CLI or I/O model.
//! All public functions:
//! - Accept typed parameters
//! - Return `Result<f64, String>` for safe error handling
//! - Perform input validation
//! - Are pure functions with no side effects
//!
//! ## Quick Start
//!
//! ```rust
//! use gananayantra::finance::tvm::future_value;
//!
//! let fv = future_value(10_000.0, 0.08, 5).unwrap();
//! assert!(fv > 14_000.0);
//! ```
//!
//! ## More Examples
//!
//! ```rust
//! use gananayantra::health::bmi;
//!
//! let result = bmi(70.0, 1.75).unwrap();
//! assert!((result - 22.86).abs() < 0.01);
//! ```
//!
//! ```rust
//! use gananayantra::space::orbital_velocity;
//!
//! let velocity = orbital_velocity(400_000.0).unwrap();
//! assert!(velocity > 7000.0); // ~7.67 km/s for LEO
//! ```

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
