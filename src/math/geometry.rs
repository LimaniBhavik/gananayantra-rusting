use std::f64::consts::PI;

pub fn triangle_area(base: f64, height: f64) -> Result<f64, String> {
    if base < 0.0 || height < 0.0 {
        return Err("Dimensions cannot be negative".into());
    }
    Ok(0.5 * base * height)
}

pub fn cube_volume(side: f64) -> Result<f64, String> {
    if side < 0.0 {
        return Err("Side cannot be negative".into());
    }
    Ok(side.powi(3))
}

pub fn sphere_volume(radius: f64) -> Result<f64, String> {
    if radius < 0.0 {
        return Err("Radius cannot be negative".into());
    }
    Ok((4.0 / 3.0) * PI * radius.powi(3))
}

pub fn cylinder_volume(radius: f64, height: f64) -> Result<f64, String> {
    if radius < 0.0 || height < 0.0 {
        return Err("Dimensions cannot be negative".into());
    }
    Ok(PI * radius.powi(2) * height)
}

pub fn slope(x1: f64, y1: f64, x2: f64, y2: f64) -> Result<f64, String> {
    if (x2 - x1).abs() < f64::EPSILON {
        return Err("Slope is undefined (vertical line)".into());
    }
    Ok((y2 - y1) / (x2 - x1))
}

pub fn circle_area(radius: f64) -> Result<f64, String> {
    if radius < 0.0 {
        return Err("Radius cannot be negative".into());
    }
    Ok(PI * radius.powi(2))
}

pub fn rectangle_area(length: f64, width: f64) -> Result<f64, String> {
    if length < 0.0 || width < 0.0 {
        return Err("Dimensions cannot be negative".into());
    }
    Ok(length * width)
}

pub fn distance_2d(x1: f64, y1: f64, x2: f64, y2: f64) -> Result<f64, String> {
    Ok(((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt())
}

pub fn sphere_surface_area(radius: f64) -> Result<f64, String> {
    if radius < 0.0 {
        return Err("Radius cannot be negative".into());
    }
    Ok(4.0 * PI * radius.powi(2))
}

pub fn pythagorean_hypotenuse(a: f64, b: f64) -> Result<f64, String> {
    if a < 0.0 || b < 0.0 {
        return Err("Side lengths cannot be negative".into());
    }
    Ok((a.powi(2) + b.powi(2)).sqrt())
}
