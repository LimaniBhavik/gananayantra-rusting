/// Calculates Carbon Footprint of AI Training.
///
/// # Arguments
/// * `gpu_power_watts` - Power consumption per GPU
/// * `num_gpus` - Number of GPUs used
/// * `training_hours` - Duration of training
/// * `pue` - Power Usage Effectiveness (Data center efficiency, typical 1.1 - 1.5)
/// * `carbon_intensity` - gCO2 per kWh (grid specific)
///
/// # Returns
/// * Total Carbon Emissions in kgCO2
pub fn calculate_ai_carbon_footprint(
    gpu_power_watts: f64,
    num_gpus: f64,
    training_hours: f64,
    pue: f64,
    carbon_intensity: f64,
) -> Result<f64, String> {
    if gpu_power_watts < 0.0 || num_gpus < 1.0 || pue < 1.0 {
        return Err("Invalid inputs".into());
    }
    let total_power_kw = (gpu_power_watts * num_gpus) / 1000.0;
    let total_energy_kwh = total_power_kw * training_hours * pue;
    Ok((total_energy_kwh * carbon_intensity) / 1000.0) // convert g to kg
}
