//! Health and fitness calculators including
//! BMI, BMR, body fat percentage, and calorie requirements.

pub mod bmi;
pub mod calorie;
pub mod body_fat;
pub mod bmr;
pub mod ideal_weight;
pub mod pace;
pub mod pregnancy;
pub mod conception;
pub mod due_date;
pub mod health_expanded;

pub use bmi::bmi;
pub use bmr::calculate_bmr;
pub use body_fat::body_fat_percentage;
pub use calorie::daily_calorie_requirement;
// Re-export new pure functions
pub use ideal_weight::calculate_ideal_weight;
pub use pace::calculate_pace;
pub use pregnancy::calculate_pregnancy_progress;
pub use conception::estimate_conception_date;
pub use due_date::estimate_due_date;
pub use health_expanded::*;
