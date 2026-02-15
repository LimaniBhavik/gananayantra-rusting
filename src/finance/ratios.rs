/// Calculates Liquidity Coverage Ratio (LCR).
///
/// # Arguments
/// * `hqla` - High-Quality Liquid Assets
/// * `net_cash_outflows` - Total net cash outflows over 30 days
///
/// # Returns
/// * LCR Percentage (Basel III requirement >= 100%)
pub fn calculate_liquidity_coverage_ratio(hqla: f64, net_cash_outflows: f64) -> Result<f64, String> {
    if hqla < 0.0 || net_cash_outflows <= 0.0 {
        return Err("Invalid inputs".into());
    }
    Ok((hqla / net_cash_outflows) * 100.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcr() {
        assert_eq!(calculate_liquidity_coverage_ratio(120.0, 100.0).unwrap(), 120.0);
    }
}
