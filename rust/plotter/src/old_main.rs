mod can;
mod serial;

use crate::can::{CanDlc, CanManager};
use crate::serial::SerialManager;

use std::collections::VecDeque;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ser = SerialManager::new("usb-can", 115200, 0, true, false, false)?;
    let mut can = CanManager::new(&mut ser);

    let mut intervals: VecDeque<f64> = VecDeque::with_capacity(50);
    let mut last_time = Instant::now();

    loop {
        // Receive a CAN frame
        let frame = can.receive_frame()?;
        let now = Instant::now();
        let dt = now.duration_since(last_time);
        last_time = now;

        // Store interval (seconds as f64)
        let dt_sec = dt.as_secs_f64();
        if intervals.len() == 50 {
            intervals.pop_front();
        }
        intervals.push_back(dt_sec);

        // Compute average Hz
        let avg_hz = if !intervals.is_empty() {
            1.0 / (intervals.iter().sum::<f64>() / intervals.len() as f64)
        } else {
            0.0
        };

        // Unpack raw data into integers
        let mut values: Vec<i32> = Vec::new();
        let mut i = 0;
        while i < frame.data.len() {
            // for now assume little endian + 2/3 byte groups depending on type
            // here I just decode as 16-bit and 24-bit ints like in Python
            if i == 0 {
                // UInt16
                let v = u16::from_le_bytes([frame.data[i], frame.data[i + 1]]) as i32;
                values.push(v);
                i += 2;
            } else if values.len() < 7 {
                // UInt24
                let v = (frame.data[i] as u32)
                    | ((frame.data[i + 1] as u32) << 8)
                    | ((frame.data[i + 2] as u32) << 16);
                values.push(v as i32);
                i += 3;
            } else {
                // Int16
                let v = i16::from_le_bytes([frame.data[i], frame.data[i + 1]]) as i32;
                values.push(v);
                i += 2;
            }
        }

        // Scale
        let pressures: Vec<f64> = values[0..7].iter().map(|v| *v as f64 / 40960.0).collect();
        let accel: Vec<f64> = values[7..10].iter().map(|v| *v as f64 / 8192.0).collect();
        let gyro: Vec<f64> = values[10..13].iter().map(|v| *v as f64 / 65.5).collect();

        // Print only for id == 1
        if frame.id == 1 {
            println!("Received from {}", frame.id);
            println!(
                "Pressure: {:?}",
                pressures
                    .iter()
                    .map(|p| format!("{:6.3}", p))
                    .collect::<Vec<_>>()
            );
            println!(
                "Accel: {:?}",
                accel
                    .iter()
                    .map(|a| format!("{:6.2}", a))
                    .collect::<Vec<_>>()
            );
            println!(
                "Gyro: {:?}",
                gyro.iter()
                    .map(|g| format!("{:6.1}", g))
                    .collect::<Vec<_>>()
            );
            println!("{:.1} Hz", avg_hz);
        }
    }
}
