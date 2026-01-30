pub fn calculate_square_footage(length_ft: f64, width_ft: f64) -> Result<f64, String> {
    if length_ft < 0.0 || width_ft < 0.0 {
        return Err("Dimensions must be positive".into());
    }
    Ok(length_ft * width_ft)
}

pub fn calculate_concrete_yards(length_ft: f64, width_ft: f64, thickness_in: f64) -> Result<f64, String> {
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
