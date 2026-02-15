use std::time::{SystemTime, UNIX_EPOCH};

pub fn calculate_percentage_of(x: f64, y: f64) -> Result<f64, String> {
    Ok((x / 100.0) * y)
}

pub fn calculate_percentage_is(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        return Err("Denominator cannot be zero".into());
    }
    Ok((x / y) * 100.0)
}

pub fn generate_random_number(min: i32, max: i32) -> Result<i32, String> {
    if min > max {
        return Err("Min cannot be greater than max".into());
    }
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros();
    Ok((seed % (max - min + 1) as u128) as i32 + min)
}

pub fn power(base: f64, exponent: f64) -> Result<f64, String> {
    Ok(base.powf(exponent))
}

pub fn nth_root(base: f64, n: f64) -> Result<f64, String> {
    if n == 0.0 {
        return Err("Root cannot be zero".into());
    }
    Ok(base.powf(1.0 / n))
}

pub fn logarithm_10(n: f64) -> Result<f64, String> {
    if n <= 0.0 {
        return Err("Logarithm undefined for non-positive numbers".into());
    }
    Ok(n.log10())
}

pub fn natural_logarithm(n: f64) -> Result<f64, String> {
    if n <= 0.0 {
        return Err("Logarithm undefined for non-positive numbers".into());
    }
    Ok(n.ln())
}

pub fn solve_quadratic(a: f64, b: f64, c: f64) -> Result<(f64, f64), String> {
    if a == 0.0 {
        return Err("Not a quadratic equation (a=0)".into());
    }
    let discriminant = b.powi(2) - 4.0 * a * c;
    if discriminant < 0.0 {
        return Err("No real roots".into());
    }
    let r1 = (-b + discriminant.sqrt()) / (2.0 * a);
    let r2 = (-b - discriminant.sqrt()) / (2.0 * a);
    Ok((r1, r2))
}

pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a.abs()
}

pub fn lcm(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        return 0;
    }
    (a * b).abs() / gcd(a, b)
}

pub fn add_fractions(n1: i64, d1: i64, n2: i64, d2: i64) -> Result<(i64, i64), String> {
    if d1 == 0 || d2 == 0 {
        return Err("Denominator cannot be zero".into());
    }
    let common_denom = d1 * d2;
    let numerator = n1 * d2 + n2 * d1;
    Ok((numerator, common_denom))
}
