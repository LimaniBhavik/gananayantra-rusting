pub struct StatsResult {
    pub mean: f64,
    pub median: f64,
    pub range: f64,
}

pub fn calculate_basic_stats(numbers: &[f64]) -> Result<StatsResult, String> {
    if numbers.is_empty() {
        return Err("Cannot calculate stats for empty list".into());
    }

    let mut nums = numbers.to_vec();
    // Sort for median
    nums.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let count = nums.len();
    let mean: f64 = nums.iter().sum::<f64>() / count as f64;

    let median = if count % 2 == 0 {
        (nums[count / 2 - 1] + nums[count / 2]) / 2.0
    } else {
        nums[count / 2]
    };

    let range = nums.last().unwrap() - nums.first().unwrap();

    Ok(StatsResult {
        mean,
        median,
        range,
    })
}

pub fn calculate_std_dev(numbers: &[f64]) -> Result<(f64, f64), String> {
    if numbers.is_empty() {
        return Err("Cannot calculate stats for empty list".into());
    }
    let count = numbers.len() as f64;
    let mean: f64 = numbers.iter().sum::<f64>() / count;
    let variance: f64 = numbers.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / count;
    Ok((variance, variance.sqrt()))
}

pub fn calculate_z_score(x: f64, mean: f64, std_dev: f64) -> Result<f64, String> {
    if std_dev == 0.0 {
        return Err("Standard deviation cannot be zero".into());
    }
    Ok((x - mean) / std_dev)
}

pub fn calculate_probability_independent(p_a: f64, p_b: f64) -> Result<(f64, f64), String> {
    if !(0.0..=1.0).contains(&p_a) || !(0.0..=1.0).contains(&p_b) {
        return Err("Probability must be between 0 and 1".into());
    }
    let p_and = p_a * p_b;
    let p_or = p_a + p_b - p_and;
    Ok((p_and, p_or))
}

pub fn permutations(n: u64, r: u64) -> Result<u64, String> {
    if r > n {
        return Err("r cannot be greater than n".into());
    }
    let mut p = 1;
    for i in (n - r + 1)..=n {
        p *= i;
    }
    Ok(p)
}

pub fn combinations(n: u64, r: u64) -> Result<u64, String> {
    if r > n {
        return Err("r cannot be greater than n".into());
    }
    let p = permutations(n, r)?;
    let mut fact_r = 1;
    for i in 1..=r {
        fact_r *= i;
    }
    Ok(p / fact_r)
}

pub fn confidence_interval_95(mean: f64, std_dev: f64, sample_size: f64) -> Result<(f64, f64), String> {
    if sample_size <= 0.0 {
        return Err("Sample size must be positive".into());
    }
    let margin = 1.96 * (std_dev / sample_size.sqrt());
    Ok((mean - margin, mean + margin))
}
