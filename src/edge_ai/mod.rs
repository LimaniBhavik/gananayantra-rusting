//! Edge AI Calculators.
//!
//! Includes calculators for:
//! * Power (Battery Life, Energy Consumption)
//! * Performance (Inference Latency, Quantization)

pub mod power;
pub mod performance;

pub use power::iot_battery_life;
pub use performance::{edge_inference_latency, quantization_memory_saving};
