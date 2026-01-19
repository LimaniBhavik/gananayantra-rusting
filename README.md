# Gananayantra-Rusting (ગણનયંત્ર)

A comprehensive multi-purpose calculator suite implemented in Rust, covering Health, Finance, Auto, Investment, Retirement, and Taxes.

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

### 3. Investment & Retirement
- **Investment**: ROI, IRR, NPV, CD Calculator, Bond Yield, Average Return.
- **Retirement**: 401k, Pension, Social Security, Roth/Traditional IRA, RMD, Annuities.

### 4. Tax & Salary
- **Taxes**: Income Tax (Simplified), Marriage Tax Penalty/Bonus, Estate Tax.
- **Advanced Tax (India)**: 
  - **Old vs New Regime**: Side-by-side comparison for FY 24-25.
  - **Deductions**: HRA Exemption, Section 80G (Donations), 80C (Max 1.5L), 80D (Health), 80DD (Dependent), 80TTA (Interest), 80U (Disabled), Transport/Education Allowances.
  - **Exemptions & Benefits**: Gratuity (Gov/Private), Leave Encashment, Interest on NSC.
  - **Business & Professional**: Presumptive Taxation (44AD/44AE/44ADA), AMT (Alternate Minimum Tax), Partners Remuneration (Sec 40b), Deferred Tax, Depreciation Calculator (Block of Assets), Agent Commission (Ad-hoc).
  - **Income Sources**: Income from House Property (NAV/Standard Deduction), Relief u/s 89 (Arrears), Loss Set-off & Carry Forward.
  - **Perquisites**: Rent Free Accommodation, Motor Car Facility, Concessional Loans.
- **Salary**: Take-Home Pay, Salary Unit Converter (Hourly/Weekly/Annual).

### 5. Advanced Math & Statistics
- **Math**: Percentage, Random Numbers, Exponents/Roots, Logarithms, Quadratic Formula, GCD/LCM, Binary/Hex.
- **Statistics**: Mean/Median/Mode, Standard Deviation, Z-Score, Probability, Permutations/Combinations, Confidence Intervals.
- **Geometry**: Area, Volume, Surface Area, Slope, Distance, Pythagorean Theorem.

### 6. Specialized Industries
- **Date & Time**: Age, Duration, Day of Week.
- **Building**: Square Footage, Concrete, BTU, Tile.
- **Science**: Density, Speed, Roman Numerals, GDP.
- **Electronics**: Ohm's Law, Resistors, Voltage Drop.

### 7. Lifestyle & Utility
- **Internet**: Password Generator, Bandwidth.
- **Utility**: GPA, Tip, Shoe Size, Sleep.
- **Weather**: Wind Chill, Heat Index.
- **Transport**: Fuel Cost, MPG, Horsepower.
- **Entertainment**: Dice Roller, Love Calculator.

### 8. Professional Network & CCTV
- **CCTV**: Lens Focal Length, Viewing Angle, Identification Settings.
- **Network**: RAID Capacity, IPv4 Subnetting, mW to dBm, Wireless Link Signal.

## 💻 Usage

### Running the CLI
Ensure you have Rust installed, then run:
```bash
cargo run
```

### Example: Calculating BMI
1. Select `1. Fitness & Health`
2. Select `1. BMI Calculator`
3. Enter Weight (kg): `70`
4. Enter Height (cm): `175`
5. Result: `BMI: 22.86 (Normal weight)`

### Example: Auto Loan
1. Select `6. Auto Industry`
2. Select `1. Auto Loan Calculator`
3. Enter Price, Down Payment, and Rate.
4. Get your monthly payment and total interest.

## 📦 Installation

Add this to your `Cargo.toml`:
```toml
[dependencies]
gananayantra-rusting = "0.3.0"
```

## 🛠️ Development

Built with ❤️ using Rust.
- **License**: MIT
- **Repository**: [LimaniBhavik/gananayantra-rusting](https://github.com/LimaniBhavik/gananayantra-rusting)
