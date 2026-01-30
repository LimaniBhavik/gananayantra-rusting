use std::f64::consts::PI;

pub fn lens_focal_length(distance_m: f64, object_width_m: f64, sensor_width_mm: f64) -> Result<f64, String> {
    if object_width_m <= 0.0 { return Err("Object width must be positive".into()); }
    Ok((sensor_width_mm * distance_m) / object_width_m)
}

pub fn camera_viewing_angle(focal_length_mm: f64, sensor_width_mm: f64) -> Result<f64, String> {
    if focal_length_mm <= 0.0 { return Err("Focal length must be positive".into()); }
    let angle = 2.0 * ((sensor_width_mm / (2.0 * focal_length_mm)).atan()) * (180.0 / PI);
    Ok(angle)
}

pub fn raid_capacity(disk_size_gb: f64, count: f64, raid_level: i32) -> Result<f64, String> {
    if count < 1.0 { return Err("Need at least 1 disk".into()); }
    match raid_level {
        0 => Ok(disk_size_gb * count), // RAID 0
        1 => Ok(disk_size_gb * (count / 2.0).floor()), // RAID 1 (pairs)
        5 => {
            if count < 3.0 { Err("RAID 5 needs at least 3 disks".into()) } else { Ok(disk_size_gb * (count - 1.0)) }
        },
        _ => Err("Unsupported RAID level".into())
    }
}

pub fn ipv4_hosts(cidr: u32) -> Result<u64, String> {
    if cidr > 32 { return Err("Invalid CIDR".into()); }
    let hosts = 2u64.pow(32 - cidr);
    if hosts < 2 { Ok(0) } else { Ok(hosts - 2) }
}

pub fn mw_to_dbm(mw: f64) -> Result<f64, String> {
    if mw <= 0.0 { return Err("Power must be positive".into()); }
    Ok(10.0 * mw.log10())
}

pub fn wireless_signal_strength(tx_power_dbm: f64, tx_gain_dbi: f64, path_loss_db: f64, rx_gain_dbi: f64) -> f64 {
    tx_power_dbm + tx_gain_dbi - path_loss_db + rx_gain_dbi
}
