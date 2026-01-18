# Gananayantra-Rusting (ગણનયંત્ર)

A comprehensive Rust-based calculator suite inspired by industry-standard tools.

## Features

### 1. Fitness & Health
- **BMI**: Body Mass Index with WHO categories.
- **Calorie/BMR**: Mifflin-St Jeor equation for metabolic rate and maintenance.
- **Body Fat**: U.S. Navy Method (circumference-based).
- **Pregnancy Tools**: Due date (Naegele's Rule) and conception estimation.

### 2. Financial
- **Business Loan**: Calculate monthly payments, total interest, and total payoff.

### 3. Math
- **Basic Operations**: Addition, subtraction, multiplication, and division.

### 4. Advertising
- **CPM**: Cost Per Mille (thousand impressions) calculator for ad campaigns.

### 5. E-Commerce
- **Discounts**: Calculate savings and sale prices.
- **Profit Margin**: Calculate gross profit and margin percentages.

## Usage

Run the CLI using Cargo:

```bash
cargo run
```

Follow the on-screen prompts to select a category and input the required data.

## Security & Safety

- **No Data Storage**: Calculations are performed locally; no personal health or financial data is stored.
- **Input Validation**: Basic checks for division by zero and valid numeric input.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
gananayantra-rusting = "0.1.1"
```
