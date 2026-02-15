//! Robotics and Automation calculators.

pub mod kinematics;
pub mod energy;

pub use kinematics::{calculate_dof_planar, motion_profile_time};
pub use energy::{calculate_robot_runtime, actuator_efficiency};
