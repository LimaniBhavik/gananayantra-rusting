# gananayantra-rusting

[![Crates.io](https://img.shields.io/crates/v/gananayantra-rusting.svg)](https://crates.io/crates/gananayantra-rusting)
[![Docs.rs](https://docs.rs/gananayantra-rusting/badge.svg)](https://docs.rs/gananayantra-rusting)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**gananayantra** is a professional-grade, multi-industry calculation engine written in Rust. It provides reusable, pure-function calculators across diverse domains including Finance, AI, Computer Vision, Edge Computing, Robotics, and more.

## Key Features

- **Library-First**: All logic is exposed as pure functions. No forced CLI or I/O.
- **Safe**: Uses `Result<T, String>` for error handling.
- **Comprehensive**: Covers 15+ industries with 100+ specialized calculators.
- **Zero-Dependency Core**: Minimal dependencies (only `chrono` for dates).

## Modules

- **Finance**: ROI, TVM, Compound Interest, Loans, Tax, Retirement, Auto Loans, Quant (Sharpe, VaR)
- **Health**: BMI, BMR, Body Fat, Pregnancy, Fitness
- **Energy**: Power Consumption, Electricity Cost
- **Physics**: Fluid Dynamics (Reynolds), Kinematics (KE/PE)
- **Logistics**: Freight Volumetric Weight, EOQ
- **Climate**: Carbon Footprint, Solar ROI, Battery Storage
- **Semiconductor**: Yields, Wafer Utilization, Costs, Moore's Law
- **AI & Compute**: Training Costs, Inference, Business ROI, Infrastructure, Operations, Development
- **Computer Vision**: Processing Times, Compression, FPS Optimization, IoU
- **NLP**: Text Analysis, Reading Time, Similarity, Compression
- **Edge AI**: IoT Battery Life, Inference Latency, Quantization
- **Robotics**: Kinematics, Energy Consumption
- **Economics**: CLV, AI ROI, Supply Chain Efficiency
- **Future Tech**: Quantum Computing Metrics
- **Data Science**: Metrics (F1, Accuracy), Statistics (Drift)
- **Geo**: Earth Distance (Haversine), Horizon
- **Water**: Pressure at Depth, Flow Rate
- **Space**: Orbital Velocity, Escape Velocity, Period, Rocketry, Satellites
- **Math**: Statistics, Geometry, Advanced Math
- **Utilities**: Lifestyle tools, Percentage, Password Gen (Basic)
- **Specialized**: Building, Electronics, Networking, Science

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
gananayantra-rusting = "1.5.0"
```

## Quick Start

```rust
use gananayantra::finance::tvm::future_value;
use gananayantra::ai::model::calculate_model_memory;

fn main() {
    // Finance
    let fv = future_value(10_000.0, 0.08, 5).unwrap();
    println!("Future Value: {:.2}", fv);

    // AI
    let mem = calculate_model_memory(70.0, 16.0, 4096.0, 8192.0, 80.0).unwrap();
    println!("AI Model Memory: {:.2} GB", mem);
}
```

## Examples

Check the `examples/` directory for usage of every module:

- `examples/finance_tvm.rs`
- `examples/ai_advanced.rs`
- `examples/computer_vision.rs`
- `examples/nlp.rs`
- `examples/edge_ai.rs`
- `examples/robotics.rs`
- `examples/economics.rs`
- `examples/data_science.rs`
- ...and many more.

Run an example:
```bash
cargo run --example ai_advanced
```

## License

MIT
