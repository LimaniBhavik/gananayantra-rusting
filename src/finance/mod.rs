//! Finance-related calculators including
//! ROI, time value of money, and compound interest.

pub mod roi;
pub mod tvm;
pub mod compound_interest;
pub mod financial;
pub mod investment_ind;
pub mod retirement_ind;
pub mod tax_salary;
pub mod sebi;
pub mod advertising;
pub mod ecommerce;
pub mod auto_loans;
pub mod quant;
pub mod options;
pub mod ratios;

pub use roi::roi;
pub use tvm::future_value;
pub use compound_interest::compound_interest;
pub use auto_loans::*;
pub use advertising::*;
pub use ecommerce::*;
pub use financial::*;
pub use investment_ind::*;
pub use retirement_ind::*;
pub use tax_salary::*;
pub use sebi::*;
pub use quant::*;
pub use options::{calculate_call_option_price, calculate_greeks};
pub use ratios::calculate_liquidity_coverage_ratio;
