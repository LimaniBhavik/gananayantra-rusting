//! Physics calculators for engineering and science.

pub mod fluid;
pub mod kinematics;

pub use fluid::calculate_reynolds_number;
pub use kinematics::{calculate_kinetic_energy, calculate_potential_energy};
