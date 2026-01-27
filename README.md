# gananayantra-rusting

A multi-industry calculation engine written in Rust.

## Features
- Finance calculators (ROI, TVM, compound interest)
- Energy calculators (power consumption, electricity cost)
- Geo calculations (Earth distance, horizon)
- Water calculations (pressure, river flow)
- Space calculations (orbital mechanics)
- Utility helpers

## Usage

```rust
use gananayantra_rusting::finance::future_value;

let fv = future_value(10_000.0, 0.08, 5).unwrap();
```

```rust
use gananayantra_rusting::space::orbital_velocity;

let v = orbital_velocity(400_000.0).unwrap();
```

## License

MIT
