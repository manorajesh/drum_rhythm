use std::io::Read;
use std::time::{Duration, Instant};

fn main() {
    let port_name = "/dev/cu.usbmodem14101"; // Update port name as needed
    let baud_rate = 9600;
    let timeout_duration = Duration::from_millis(10); // 100ms timeout to detect end of burst

    let mut port = serialport::new(port_name, baud_rate)
        .timeout(timeout_duration)
        .open()
        .expect("Failed to open port");

    let mut buf = [0; 1];
    let mut read_buf = String::new();
    let mut max_velocity = 0;
    let mut last_received = Instant::now();
    let mut in_burst = false;

    loop {
        match port.read(&mut buf) {
            Ok(n) => {
                last_received = Instant::now();
                let s = std::str::from_utf8(&buf[0..n]).unwrap();
                read_buf.push_str(s);

                if let Some(newline_pos) = read_buf.find('\n') {
                    let line = read_buf.drain(..=newline_pos).collect::<String>();
                    if let Ok(sensor_value) = line.trim().parse::<i32>() {
                        in_burst = true;
                        max_velocity = max_velocity.max(sensor_value);
                    }
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                if in_burst && last_received.elapsed() >= timeout_duration {
                    let clamped_velocity = max_velocity.normalize(100, 1023, 0, 1024);
                    println!("Hit: {}", clamped_velocity);
                    max_velocity = 0;
                    in_burst = false;
                }
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}

trait Normalize {
    fn normalize(self, from_min: Self, from_max: Self, to_min: Self, to_max: Self) -> Self;
}

impl Normalize for i32 {
    fn normalize(self, from_min: Self, from_max: Self, to_min: Self, to_max: Self) -> Self {
        let from_range = from_max - from_min;
        let to_range = to_max - to_min;
        let scaled = (self - from_min) as f64 / from_range as f64;
        (to_min as f64 + (scaled * to_range as f64)) as i32
    }
}