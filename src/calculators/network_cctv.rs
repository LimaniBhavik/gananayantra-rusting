use crate::calculators::utils::read_input;

pub fn run_menu() {
    loop {
        println!("\n--- Network & CCTV Tools ---");
        println!("1. Lens Focal Length Calculator");
        println!("2. Camera Viewing Angle");
        println!("3. RAID Capacity Calculator");
        println!("4. IPv4 Subnet Calculator");
        println!("5. mW to dBm Converter");
        println!("6. Wireless Link Signal Calculator");
        println!("0. Back");
        let choice = read_input("Select an option: ");

        match choice as i32 {
            1 => lens_calculator(),
            2 => viewing_angle(),
            3 => raid_calculator(),
            4 => ipv4_subnet(),
            5 => mw_to_dbm(),
            6 => wireless_signal(),
            0 => break,
            _ => println!("Invalid choice."),
        }
    }
}

fn lens_calculator() {
    println!("\n--- Lens Focal Length ---");
    let dist = read_input("Distance to object (m): ");
    let width = read_input("Width of object (m): ");
    let sensor_width = 4.8; // standard 1/3" sensor width in mm
    let f = (sensor_width * dist) / width;
    println!("Required Focal Length: {:.2} mm", f);
}

fn viewing_angle() {
    println!("\n--- Viewing Angle ---");
    let f = read_input("Focal Length (mm): ");
    let sensor_width = 4.8; 
    let angle = 2.0 * ( (sensor_width / (2.0 * f)).atan() ) * (180.0 / std::f64::consts::PI);
    println!("Horizontal Viewing Angle: {:.2}°", angle);
}

fn raid_calculator() {
    println!("\n--- RAID Capacity ---");
    let size = read_input("Single Disk Size (GB): ");
    let count = read_input("Number of Disks: ");
    println!("1. RAID 0 (Striping)");
    println!("2. RAID 1 (Mirroring)");
    println!("3. RAID 5 (Parity)");
    let choice = read_input("Choice: ");
    
    match choice as i32 {
        1 => println!("Usable Capacity: {:.2} GB", size * count),
        2 => println!("Usable Capacity: {:.2} GB", size * (count / 2.0)),
        3 => println!("Usable Capacity: {:.2} GB", size * (count - 1.0)),
        _ => println!("Invalid."),
    }
}

fn ipv4_subnet() {
    println!("\n--- IPv4 Subnet Info ---");
    let cidr = read_input("CIDR Prefix (e.g., 24): ");
    let hosts = 2.0_f64.powf(32.0 - cidr) - 2.0;
    println!("Number of usable hosts: {:.0}", hosts.max(0.0));
}

fn mw_to_dbm() {
    let mw = read_input("Power in mW: ");
    let dbm = 10.0 * mw.log10();
    println!("Power in dBm: {:.2} dBm", dbm);
}

fn wireless_signal() {
    let tx_power = read_input("TX Power (dBm): ");
    let tx_gain = read_input("TX Antenna Gain (dBi): ");
    let path_loss = read_input("Free Space Path Loss (dB): ");
    let rx_gain = read_input("RX Antenna Gain (dBi): ");
    let signal = tx_power + tx_gain - path_loss + rx_gain;
    println!("Received Signal Level: {:.2} dBm", signal);
}
