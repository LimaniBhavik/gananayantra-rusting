use gananayantra::computer_vision::processing::{video_processing_time, image_compression_ratio, max_supported_fps};

fn main() {
    println!("--- Computer Vision Calculators ---");

    // Video Processing Time
    // 1 hour video, 30 fps, model takes 16ms per frame (approx 60fps speed)
    let duration = 3600.0;
    let fps = 30.0;
    let proc_time_ms = 16.66;

    match video_processing_time(duration, fps, proc_time_ms) {
        Ok(time) => println!("Time to process 1h video: {:.2} seconds ({:.2} minutes)", time, time / 60.0),
        Err(e) => println!("Error: {}", e),
    }

    // Compression
    let raw_kb = 5000.0;
    let compressed_kb = 200.0;
    match image_compression_ratio(raw_kb, compressed_kb) {
        Ok(ratio) => println!("Compression Ratio: {:.1}:1", ratio),
        Err(e) => println!("Error: {}", e),
    }

    // Max FPS
    match max_supported_fps(proc_time_ms) {
        Ok(max_fps) => println!("Max Supported FPS: {:.1}", max_fps),
        Err(e) => println!("Error: {}", e),
    }
}
