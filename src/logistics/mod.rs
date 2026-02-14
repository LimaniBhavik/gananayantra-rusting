//! Logistics and Supply Chain calculators.

pub mod freight;
pub mod inventory;

pub use freight::calculate_volumetric_weight;
pub use inventory::calculate_eoq;
