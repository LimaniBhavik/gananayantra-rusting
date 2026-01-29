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
