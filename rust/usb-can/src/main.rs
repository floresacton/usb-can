mod can;
mod serial;

use crate::can::CanManager;
use crate::serial::SerialManager;
use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

const HISTORY_SIZE: usize = 1000;

#[derive(Clone)]
struct DataHistory {
    pressures: Vec<VecDeque<[f64; 2]>>, // [time, value] for 7 sensors
    accel: Vec<VecDeque<[f64; 2]>>,     // [time, value] for 3 axes
    gyro: Vec<VecDeque<[f64; 2]>>,      // [time, value] for 3 axes
    hz: f64,
}

impl DataHistory {
    fn new() -> Self {
        Self {
            pressures: (1..7)
                .map(|_| VecDeque::with_capacity(HISTORY_SIZE))
                .collect(),
            accel: (0..3)
                .map(|_| VecDeque::with_capacity(HISTORY_SIZE))
                .collect(),
            gyro: (0..3)
                .map(|_| VecDeque::with_capacity(HISTORY_SIZE))
                .collect(),
            hz: 0.0,
        }
    }

    fn add_data(&mut self, time: f64, pressures: &[f64], accel: &[f64], gyro: &[f64], hz: f64) {
        for (i, &p) in pressures.iter().enumerate() {
            if self.pressures[i].len() >= HISTORY_SIZE {
                self.pressures[i].pop_front();
            }
            self.pressures[i].push_back([time, p]);
        }

        for (i, &a) in accel.iter().enumerate() {
            if self.accel[i].len() >= HISTORY_SIZE {
                self.accel[i].pop_front();
            }
            self.accel[i].push_back([time, a]);
        }

        for (i, &g) in gyro.iter().enumerate() {
            if self.gyro[i].len() >= HISTORY_SIZE {
                self.gyro[i].pop_front();
            }
            self.gyro[i].push_back([time, g]);
        }

        self.hz = hz;
    }
}

struct PlotterApp {
    history: Arc<Mutex<DataHistory>>,
}

impl PlotterApp {
    fn new(history: Arc<Mutex<DataHistory>>) -> Self {
        Self { history }
    }
}

impl eframe::App for PlotterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint(); // Continuous updates

        let history = self.history.lock().unwrap().clone();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(format!("CAN Data Monitor - {:.1} Hz", history.hz));

            // Pressure Plot
            ui.label("Pressure Sensors");
            Plot::new("pressure_plot")
                .height(250.0)
                .y_axis_label("Pressure")
                .legend(egui_plot::Legend::default())
                .show(ui, |plot_ui| {
                    let colors = [
                        egui::Color32::RED,
                        egui::Color32::BLUE,
                        egui::Color32::GREEN,
                        egui::Color32::from_rgb(255, 0, 255), // Magenta
                        egui::Color32::CYAN,
                        egui::Color32::YELLOW,
                        egui::Color32::BLACK,
                    ];
                    for (i, data) in history.pressures.iter().enumerate() {
                        let points: PlotPoints = data.iter().copied().collect();
                        plot_ui.line(Line::new(format!("P{}", i), points).color(colors[i]));
                    }
                });

            ui.separator();

            // Accelerometer Plot
            ui.label("Accelerometer");
            Plot::new("accel_plot")
                .height(250.0)
                .y_axis_label("Acceleration (g)")
                .legend(egui_plot::Legend::default())
                .show(ui, |plot_ui| {
                    let colors = [
                        egui::Color32::RED,
                        egui::Color32::BLUE,
                        egui::Color32::GREEN,
                    ];
                    let labels = ["X", "Y", "Z"];
                    for (i, data) in history.accel.iter().enumerate() {
                        let points: PlotPoints = data.iter().copied().collect();
                        plot_ui.line(Line::new(labels[i], points).color(colors[i]));
                    }
                });

            ui.separator();

            // Gyroscope Plot
            ui.label("Gyroscope");
            Plot::new("gyro_plot")
                .height(250.0)
                .y_axis_label("Angular Rate (°/s)")
                .legend(egui_plot::Legend::default())
                .show(ui, |plot_ui| {
                    let colors = [
                        egui::Color32::RED,
                        egui::Color32::BLUE,
                        egui::Color32::GREEN,
                    ];
                    let labels = ["X", "Y", "Z"];
                    for (i, data) in history.gyro.iter().enumerate() {
                        let points: PlotPoints = data.iter().copied().collect();
                        plot_ui.line(Line::new(labels[i], points).color(colors[i]));
                    }
                });
        });
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let history = Arc::new(Mutex::new(DataHistory::new()));
    let history_clone = Arc::clone(&history);

    // Spawn CAN reading thread
    thread::spawn(move || {
        if let Err(e) = can_reader_thread(history_clone) {
            eprintln!("CAN reader error: {}", e);
        }
    });

    // Start GUI
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 900.0]),
        ..Default::default()
    };

    eframe::run_native(
        "CAN Data Plotter",
        options,
        Box::new(|_cc| Ok(Box::new(PlotterApp::new(history)))),
    )?;

    Ok(())
}

fn can_reader_thread(history: Arc<Mutex<DataHistory>>) -> Result<(), Box<dyn std::error::Error>> {
    let mut ser = SerialManager::new("usb-can", 115200, 0, true, false, false)?;
    let mut can = CanManager::new(&mut ser);
    let mut intervals: VecDeque<f64> = VecDeque::with_capacity(50);
    let mut last_time = Instant::now();
    let start_time = Instant::now();

    loop {
        // Receive a CAN frame
        let frame = can.receive_frame()?;
        let now = Instant::now();
        let dt = now.duration_since(last_time);
        last_time = now;

        // Store interval
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

        // Unpack raw data
        let mut values: Vec<i32> = Vec::new();
        let mut i = 0;
        while i < frame.data.len() {
            if i == 0 {
                let v = u16::from_le_bytes([frame.data[i], frame.data[i + 1]]) as i32;
                values.push(v);
                i += 2;
            } else if values.len() < 7 {
                let v = (frame.data[i] as u32)
                    | ((frame.data[i + 1] as u32) << 8)
                    | ((frame.data[i + 2] as u32) << 16);
                values.push(v as i32);
                i += 3;
            } else {
                let v = i16::from_le_bytes([frame.data[i], frame.data[i + 1]]) as i32;
                values.push(v);
                i += 2;
            }
        }

        // Scale
        let pressures: Vec<f64> = values[1..7].iter().map(|v| *v as f64 / 40960.0).collect();
        let accel: Vec<f64> = values[7..10].iter().map(|v| *v as f64 / 8192.0).collect();
        let gyro: Vec<f64> = values[10..13].iter().map(|v| *v as f64 / 65.5).collect();

        // Process only for id == 1
        if frame.id == 1 {
            let elapsed = start_time.elapsed().as_secs_f64();
            let mut hist = history.lock().unwrap();
            hist.add_data(elapsed, &pressures, &accel, &gyro, avg_hz);
        }
    }
}
