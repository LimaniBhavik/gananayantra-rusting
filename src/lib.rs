//! # gananayantra
//!
//! **gananayantra** is a professional-grade, multi-industry calculation engine written in Rust.
//! It provides reusable, library-first calculators across diverse domains including:
//!
//! - **Finance**: ROI, time value of money, compound interest, loans, taxes, retirement
//! - **Health**: BMI, BMR, body fat percentage, pregnancy, fitness
//! - **Energy**: Power consumption, electricity cost
//! - **Geo**: Earth distance (Haversine), horizon distance
//! - **Water**: Pressure at depth, river flow rate
//! - **Space**: Orbital velocity, escape velocity, orbital period
//! - **Math**: Statistics, geometry, advanced math
//! - **Utilities**: Percentage calculations, lifestyle tools
//! - **Specialized**: Building, electronics, networking, science
//!
//! ## Design Principles
//!
//! This crate is **library-first** and does not impose any CLI or I/O model.
//! All public functions:
//! - Accept typed parameters
//! - Return `Result<T, String>` for safe error handling (usually `Result<f64, String>`)
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

pub mod climate;
pub mod energy;
pub mod finance;
pub mod finance_investment;
pub mod geo;
pub mod health;
pub mod math;
pub mod space;
pub mod specialized;
pub mod utilities;
pub mod utility_lifestyle;
pub mod water;
