# Gananayantra-Rusting (ગણનયંત્ર)

## Overview
Gananayantra-Rusting is a high-performance, modular calculator engine built in Rust. It serves as a comprehensive suite for diverse industries, providing accurate and verifiable calculations through a clean CLI interface and a reusable library architecture.

## Architecture (v1.0.2)
The project follows a **Modular Industry-Based Structure**. Each major industry has its own root module containing specific calculators as sub-modules.

### Directory Structure
```
src/
├── main.rs                 # CLI Entry Point & Global Menu
├── calculators/            # Shared Utilities & Legacy Base
├── health/                 # BMI, BMR, Pregnancy, etc.
├── finance/                # Loan, Tax, Retirement, SEBI, Ads, E-Com
├── math/                   # Basic, Advanced, Stats, Geometry
├── auto/                   # Automotive Industry Tools
├── specialized/            # Science, Electronics, Building, CCTV
├── utility_lifestyle/      # Internet, Weather, Transport, Utility
├── energy/                 # [NEW] Power, Solar, Consumption
├── geo/                    # [NEW] Distance, Earth Curvature
├── water/                  # [NEW] Pressure, Flow Rate
└── space/                  # [NEW] Orbital, Satellite, Escape Velocity
```

## Naming & Quality Standards
1. **蛇形命名 (snake_case)**: All functions and variables must use snake_case.
2. **Pure Logic**: Logic functions must be pure, return `Result<T, String>`, and handle no I/O.
3. **CLI Separation**: Input/Output handling must live in `mod.rs` or `main.rs`, never in the logic function.
4. **Validation**: All inputs must be validated for physical/mathematical limits.
5. **Testing**: Every calculator must have `#[test]` coverage for valid, boundary, and error cases.

## Roadmap
- [x] Phase 1: Core Financial & Health Engines (v1.0.0)
- [x] Phase 2: Professional Tax & Compliance (v1.0.2)
- [ ] Phase 3: Energy & Geo-Spatial Modules (Upcoming)
- [ ] Phase 4: Space & Astronomy Modules
- [ ] Phase 5: Crate API Stabilization for Library Use

## User Preferences
- **Replit-First**: All development and testing happens directly in the Replit environment.
- **Minimal Dependencies**: Use standard library features whenever possible.
- **Clean CLI**: Maintain a structured, numbered menu system.
