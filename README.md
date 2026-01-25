# Gananayantra-Rusting (ગણનયંત્ર) v1.1.0

A professional-grade, modular multi-purpose calculator suite implemented in Rust.

## 🌟 Latest Updates (v1.1.0)
- **New Energy Module**: Power Consumption and Electricity Cost calculators.
- **New Geo Module**: Earth Distance (Haversine) and Horizon Distance (Earth Curvature).
- **New Water Module**: Pressure at Depth and River Flow Rate.
- **New Space Module**: Orbital Velocity, Escape Velocity, and Satellite Orbital Period.
- **Expanded Finance Module**: ROI, TVM (Future Value), and Compound Interest.
- **New Utilities Module**: Percentage Calculator ($X\%$ of $Y$).
- **Full Indian Tax Suite**: Comprehensive compliance for FY 24-25.

## 🧱 Project Structure
The engine is divided into logical industry modules:
- **Finance**: ROI, TVM, Compound Interest, Loans, Tax, E-Commerce.
- **Health**: BMI, BMR, TDEE, Pregnancy.
- **Math**: Basic, Advanced, Statistics, Geometry.
- **Energy**: Power Consumption, Electricity Cost.
- **Geo**: Earth Distance, Horizon Distance.
- **Water**: Pressure at Depth, River Flow Rate.
- **Space**: Orbital Velocity, Escape Velocity, Orbital Period.
- **Utilities**: Percentage calculations.
- **Specialized**: Electronics, Building, Network & CCTV.

## 🚀 Scalability Pattern
Every calculator follows a strict 3-tier structure:
1. **Pure Logic**: Isolated function in `<industry>/<calculator>.rs`.
2. **CLI Wrapper**: Standardized input/output handling.
3. **Unit Tests**: Integrated `#[test]` coverage for every calculation.

## 🚀 Features

### 1. Fitness & Health (WHO & Industry Standards)
- **Classic**: BMI, Calories, BMR, Body Fat (Navy Method), Ideal Weight, Pace.
- **Maternal**: Pregnancy Due Date, Conception, Weight Gain.
- **Expanded**: Healthy Weight Range, Calories Burned, One Rep Max, Target Heart Rate, TDEE, Macros, Ovulation, BAC, BSA, Lean Body Mass.

### 2. Financial & Loans
- **Core**: Mortgage, Auto Loan, Business Loan, Simple/Compound Interest.
- **Real Estate**: Refinance, Rental Property, House Affordability, Rent vs Buy, VA/FHA Loans, HELOC.
- **Debt**: Credit Card Payoff, Debt Consolidation, Student Loans.
- **Business**: VAT, Depreciation, Commissions, Amortization.
- **Investment**: ROI, TVM (Future Value), Compound Interest.

### 3. Energy & Utilities
- **Energy**: Power Consumption (kWh), Electricity Cost.
- **Utilities**: Percentage Calculator.

### 4. Geo, Water & Space
- **Geo**: Earth Distance (Haversine), Horizon Distance (Earth Curvature).
- **Water**: Pressure at Depth, River Flow Rate.
- **Space**: Orbital Velocity, Escape Velocity, Satellite Orbital Period.

### 5. Tax & Salary (India Compliance)
- **Old vs New Regime**: Side-by-side comparison for FY 24-25.
- **Deductions**: HRA, Section 80C, 80D, 80G, 80TTA, 80U, etc.
- **Business**: Presumptive Taxation (44AD/ADA/AE), AMT, Partners Remuneration.
- **Salary**: Take-Home Pay, Salary Unit Converter.

### 6. Advanced Math & Statistics
- **Math**: Random Numbers, Exponents, Logarithms, GCD/LCM, Binary/Hex.
- **Statistics**: Mean/Median/Mode, Standard Deviation, Probability.
- **Geometry**: Area, Volume, Surface Area, Pythagorean Theorem.

## 💻 Usage

### Running the CLI
Ensure you have Rust installed, then run:
```bash
cargo run
```

### Installation
Add this to your `Cargo.toml`:
```toml
[dependencies]
gananayantra-rusting = "1.1.0"
```

## 🛠️ Development
Built with ❤️ using Rust.
- **License**: MIT
- **Repository**: [LimaniBhavik/gananayantra-rusting](https://github.com/LimaniBhavik/gananayantra-rusting)
