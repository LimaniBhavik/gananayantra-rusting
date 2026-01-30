pub fn calculate_square_footage(length_ft: f64, width_ft: f64) -> Result<f64, String> {
    if length_ft < 0.0 || width_ft < 0.0 {
        return Err("Dimensions must be positive".into());
    }
    Ok(length_ft * width_ft)
}

pub fn calculate_concrete_yards(
    length_ft: f64,
    width_ft: f64,
    thickness_in: f64,
) -> Result<f64, String> {
    if length_ft < 0.0 || width_ft < 0.0 || thickness_in < 0.0 {
        return Err("Dimensions must be positive".into());
    }
    let cubic_yards = (length_ft * width_ft * (thickness_in / 12.0)) / 27.0;
    Ok(cubic_yards)
}

pub fn calculate_btu_cooling(area_sq_ft: f64) -> Result<f64, String> {
    if area_sq_ft < 0.0 {
        return Err("Area must be positive".into());
    }
    Ok(area_sq_ft * 20.0)
}

pub fn calculate_tiles_needed(area_sq_ft: f64, tile_size_sq_in: f64) -> Result<(f64, f64), String> {
    if area_sq_ft < 0.0 || tile_size_sq_in <= 0.0 {
        return Err("Invalid inputs".into());
    }
    let count = (area_sq_ft * 144.0) / tile_size_sq_in;
    Ok((count.ceil(), (count * 1.1).ceil()))
}

/// Calculates gallons of paint needed for wall area.
/// Assumes standard coverage of ~350-400 sq ft per gallon.
pub fn calculate_paint_gallons(area_sq_ft: f64, coats: f64) -> Result<f64, String> {
    if area_sq_ft < 0.0 || coats < 1.0 {
        return Err("Invalid input values".into());
    }
    let coverage_per_gallon = 350.0;
    Ok((area_sq_ft * coats) / coverage_per_gallon)
}

/// Calculates Board Feet for lumber.
/// Formula: (Thickness(in) * Width(in) * Length(ft)) / 12
pub fn calculate_board_feet(
    thickness_in: f64,
    width_in: f64,
    length_ft: f64,
    quantity: u32,
) -> Result<f64, String> {
    if thickness_in <= 0.0 || width_in <= 0.0 || length_ft <= 0.0 {
        return Err("Dimensions must be positive".into());
    }
    let bf_per_piece = (thickness_in * width_in * length_ft) / 12.0;
    Ok(bf_per_piece * quantity as f64)
}
