# Gananayantra-Rusting

## Overview

Gananayantra-Rusting (ગણનયંત્ર - Calculator in Gujarati) is a Rust-based calculator application inspired by calculator.net. The project aims to provide calculator functionality similar to the popular online calculator service, implemented in the Rust programming language.

## User Preferences

Preferred communication style: Simple, everyday language.

## System Architecture

### Language and Build System
- **Language**: Rust (rustc 1.88.0)
- **Build System**: Cargo (standard Rust package manager and build tool)
- **Project Type**: Binary application (`bin-gananayantra-rusting`)

### Core Architecture
The project follows a standard Rust binary application structure:
- Single binary target compiled from source
- Uses Cargo's default project layout
- Debug and test configurations available

### Dependencies
Based on the build fingerprints, the project uses:
- **chrono**: Date and time library with features including `clock`, `std`, `alloc`, and timezone support (`iana-time-zone`)
- **num-traits**: Numeric traits for generic mathematics operations
- **autocfg**: Build-time configuration detection

### Design Decisions

1. **Rust for Implementation**
   - Problem: Need for a performant, safe calculator application
   - Solution: Rust provides memory safety without garbage collection and excellent performance
   - Pros: Type safety, zero-cost abstractions, reliable error handling
   - Cons: Steeper learning curve compared to scripting languages

2. **Chrono for Time Handling**
   - Problem: Calculator may need date/time calculations or timestamps
   - Solution: Using the chrono crate with full timezone support via iana-time-zone
   - Pros: Comprehensive time handling, well-maintained library
   - Cons: Adds to binary size

## External Dependencies

### Crate Dependencies
| Crate | Purpose |
|-------|---------|
| chrono | Date and time handling with timezone support |
| num-traits | Generic numeric type traits for mathematical operations |
| iana-time-zone | IANA timezone database support (fallback feature enabled) |
| autocfg | Compile-time feature detection |

### External Services
- None currently integrated

### Database
- None currently integrated

### Build Requirements
- Rust toolchain (1.88.0 or compatible)
- Cargo package manager