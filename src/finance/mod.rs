//! Finance-related calculators including
//! ROI, time value of money, and compound interest.

pub mod advertising;
pub mod auto_loans;
pub mod compound_interest;
pub mod ecommerce;
pub mod financial;
pub mod investment_ind;
pub mod retirement_ind;
pub mod roi;
pub mod sebi;
pub mod tax_salary;
pub mod tvm;

pub use advertising::*;
pub use auto_loans::*;
pub use compound_interest::compound_interest;
pub use ecommerce::*;
pub use roi::roi;
pub use tvm::future_value;
// `financial` module is massive, so we might want to keep itnamespaced or export specific items if desired.
// For now, let's export its content to flatten the API for ease of use as per "Library-first" philosophy.
pub use financial::*;
pub use investment_ind::*;
pub use retirement_ind::*;
pub use sebi::*;
pub use tax_salary::*;
