//! AI and Compute Infrastructure calculators.

pub mod cost;
pub mod energy;
pub mod cluster;
pub mod model;
pub mod training;
pub mod inference;
pub mod applications;
pub mod business;
pub mod development;
pub mod infrastructure;
pub mod operations;

pub use cost::{calculate_training_cost, calculate_inference_cost};
pub use energy::calculate_ai_carbon_footprint;
pub use cluster::{calculate_gpu_cluster_cost, calculate_pue};
pub use model::{calculate_model_memory, attention_complexity, quantization_saving};
pub use training::{estimate_training_time, learning_rate_decay, estimate_epochs};
pub use inference::{predict_latency, calculate_throughput};

// New additions
pub use applications::{chatbot_response_time, recommendation_accuracy, clv_prediction};
pub use business::{cost_per_prediction, revenue_impact, supply_chain_efficiency};
pub use development::{calculate_dataset_size, batch_size_optimizer, fine_tuning_cost};
pub use infrastructure::{cloud_compute_cost, data_storage_projection, predict_bandwidth_usage, cooling_requirements_btu, storage_iops};
pub use operations::{calculate_token_cost, context_window_efficiency, parameter_efficiency, memory_usage_optimization};
