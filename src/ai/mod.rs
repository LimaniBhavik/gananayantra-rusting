//! AI and Compute Infrastructure calculators.

pub mod cost;
pub mod energy;
pub mod cluster;

pub use cost::{calculate_training_cost, calculate_inference_cost};
pub use energy::calculate_ai_carbon_footprint;
pub use cluster::{calculate_gpu_cluster_cost, calculate_pue};
