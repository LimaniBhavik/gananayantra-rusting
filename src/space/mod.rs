//! Space and orbital mechanics calculators including
//! orbital velocity, escape velocity, and orbital period.

pub mod orbital_velocity;
pub mod escape_velocity;
pub mod orbital_period;

pub use orbital_velocity::orbital_velocity;
pub use escape_velocity::escape_velocity;
pub use orbital_period::orbital_period;
