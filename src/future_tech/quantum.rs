/// Calculates Quantum Volume Approximation.
///
/// # Arguments
/// * `qubits` - Number of physical qubits
/// * `error_rate` - Gate error rate
///
/// # Returns
/// * Approximate Quantum Volume (2^min(N, 1/error))
pub fn calculate_quantum_volume(qubits: f64, error_rate: f64) -> Result<f64, String> {
    if error_rate <= 0.0 || error_rate >= 1.0 { return Err("Invalid error rate".into()); }
    let effective_width = 1.0 / error_rate; // Rough depth limit
    let effective_qubits = qubits.min(effective_width);
    Ok(2.0_f64.powf(effective_qubits))
}

/// Estimates logical qubit requirement (with error correction).
///
/// # Arguments
/// * `physical_qubits_per_logical` - Overhead factor (e.g., 1000:1 for surface code)
/// * `required_logical_qubits` - Target logical qubits
///
/// # Returns
/// * Total Physical Qubits
pub fn physical_qubit_requirement(physical_qubits_per_logical: f64, required_logical_qubits: f64) -> Result<f64, String> {
    Ok(physical_qubits_per_logical * required_logical_qubits)
}
