# gananayantra-rusting

[![Crates.io](https://img.shields.io/crates/v/gananayantra-rusting.svg)](https://crates.io/crates/gananayantra-rusting)
[![Docs.rs](https://docs.rs/gananayantra-rusting/badge.svg)](https://docs.rs/gananayantra-rusting)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**gananayantra** is a professional-grade, multi-industry calculation engine written in Rust. It provides reusable, pure-function calculators across diverse domains including Finance, Health, Energy, Physics, and more.

## Key Features

- **Library-First**: All logic is exposed as pure functions. No forced CLI or I/O.
- **Safe**: Uses `Result<T, String>` for error handling.
- **Comprehensive**: Covers 10+ industries with 50+ calculators.
- **Zero-Dependency Core**: Minimal dependencies (only `chrono` for dates).

## Modules

- **Finance**: ROI, TVM, Compound Interest, Loans, Tax, Retirement, Auto Loans, Quant (Sharpe, VaR)
- **Health**: BMI, BMR, Body Fat, Pregnancy, Fitness
- **Energy**: Power Consumption, Electricity Cost
- **Physics**: Fluid Dynamics (Reynolds), Kinematics (KE/PE)
- **Logistics**: Freight Volumetric Weight, EOQ
- **Climate**: Carbon Footprint, Solar ROI, Battery Storage
- **Semiconductor**: Die Yield, Wafer Utilization, Defect Density
- **AI & Compute**: Training Costs, Inference Costs, Carbon Footprint
- **Geo**: Earth Distance (Haversine), Horizon
- **Water**: Pressure at Depth, Flow Rate
- **Space**: Orbital Velocity, Escape Velocity, Period, Rocketry (Delta-V)
- **Math**: Statistics, Geometry, Advanced Math
- **Utilities**: Lifestyle tools, Percentage, Password Gen (Basic)
- **Specialized**: Building, Electronics, Networking, Science

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
gananayantra-rusting = "1.3.0"
```

## Quick Start

```rust
use gananayantra::finance::tvm::future_value;

fn main() {
    let fv = future_value(10_000.0, 0.08, 5).unwrap();
    println!("Future Value: {:.2}", fv);
}
```

## Examples

Check the `examples/` directory for usage of every module:

- `examples/finance_tvm.rs`
- `examples/physics.rs`
- `examples/logistics.rs`
- `examples/climate.rs`
- `examples/semiconductor.rs`
- `examples/ai_compute.rs`
- ...and many more.

Run an example:
```bash
cargo run --example physics
```

## License

MIT
