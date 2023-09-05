use std::io::Read;

use serialport::TTYPort;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut port = serialport::new("/dev/cu.usbmodem14601", 9600)
        .timeout(std::time::Duration::from_millis(1000))
        .open_native()?;

    // let mut packet_buf = Vec::new();
    let mut serial_buf = [0; 1];

    // let baseline = get_baseline(&mut port);

    loop {
        match port.read(&mut serial_buf) {
            Ok(t) => {
                for &byte in serial_buf[0..t].iter() {
                    // if byte == b'\n' {
                    //     process_packet(&packet_buf, baseline);
                    //     packet_buf.clear();
                    // } else {
                    //     packet_buf.push(byte);
                    // }

                    let ch = byte as char;

                    if ch.is_numeric() {
                        // let num = ch.to_digit(10).unwrap();
                        if ch != '0' {
                            println!("Hit: {}", ch);
                        }
                    } else if ch == '\n' {
                        // println!();
                    }
                }
            },
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
            Err(e) => return Err(Box::new(e)),
        }
    }
}

fn process_packet(packet: &[u8], baseline: u8) {
    let max = *packet.iter().max().unwrap_or(&0);
    if max > baseline {
        println!("max: {}", max as char);
    }
}

fn get_baseline(port: &mut TTYPort) -> u8 {
    let mut serial_buf: Vec<u8> = vec![0; 1024];  // Buffer to store incoming data
    let mut packet_buf = Vec::new();

    loop {
        match port.read(&mut serial_buf) {
            Ok(t) => {
                for &byte in serial_buf[0..t].iter() {
                    if byte == b'\n' {
                        process_packet(&packet_buf, 0);
                        packet_buf.clear();
                    } else {
                        packet_buf.push(byte);
                    }
                }
                break;
            },
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
            Err(e) => panic!("Error: {}", e),
        }
    }

    *packet_buf.iter().max().unwrap_or(&0)
}