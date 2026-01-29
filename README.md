# gananayantra-rusting

[![Crates.io](https://img.shields.io/crates/v/gananayantra-rusting.svg)](https://crates.io/crates/gananayantra-rusting)
[![Documentation](https://docs.rs/gananayantra-rusting/badge.svg)](https://docs.rs/gananayantra-rusting)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A professional-grade, multi-industry calculation engine written in Rust.

## Overview

**gananayantra** provides reusable, library-first calculators across diverse domains. All functions are pure, return `Result<f64, String>`, and perform input validation.

**This is a library crate. No CLI is included.**

## Features by Industry

| Industry | Calculators |
|----------|-------------|
| **Finance** | ROI, Time Value of Money, Compound Interest |
| **Health** | BMI, BMR, Body Fat %, Calorie Requirements |
| **Energy** | Power Consumption, Electricity Cost |
| **Geo** | Earth Distance (Haversine), Horizon Distance |
| **Water** | Pressure at Depth, River Flow Rate |
| **Space** | Orbital Velocity, Escape Velocity, Orbital Period |
| **Utilities** | Percentage Calculations |

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
gananayantra-rusting = "1.1"
```

## Usage Examples

### Finance: Future Value Calculation

```rust
use gananayantra::finance::tvm::future_value;

let fv = future_value(10_000.0, 0.08, 5).unwrap();
println!("Future value: ${:.2}", fv); // ~$14,693.28
```

### Health: BMI Calculation

```rust
use gananayantra::health::bmi;

let result = bmi(70.0, 1.75).unwrap();
println!("BMI: {:.2}", result); // ~22.86
```

### Space: Orbital Velocity

```rust
use gananayantra::space::orbital_velocity;

let velocity = orbital_velocity(400_000.0).unwrap();
println!("LEO velocity: {:.2} m/s", velocity); // ~7,670 m/s
```

## API Design

All public functions follow these principles:

- **Pure functions**: No side effects or I/O
- **Type-safe**: Strong typing with validation
- **Error handling**: Returns `Result<f64, String>`
- **Documented**: Rustdoc comments with examples

## License

MIT License - see [LICENSE](LICENSE) for details.

## Repository

[GitHub: LimaniBhavik/gananayantra-rusting](https://github.com/LimaniBhavik/gananayantra-rusting)
