use serialport::SerialPort;
use std::error::Error;
use std::time::Duration;

pub struct SerialManager {
    pub port: Box<dyn SerialPort>, // expose port so CanManager can use it
}

impl SerialManager {
    pub fn new(
        device: &str,
        baud: u32,
        device_number: usize,
        is_desc: bool,
        startswith: bool,
        print_devices: bool,
    ) -> Result<Self, Box<dyn Error>> {
        let ports = serialport::available_ports()?;

        let mut found: Option<String> = None;
        let mut remaining = device_number;

        for p in ports {
            if print_devices {
                println!("Device: {} : {:?}", p.port_name, p.port_type);
            }

            let mut is_device = false;
            match p.port_type {
                serialport::SerialPortType::UsbPort(info) => {
                    if is_desc {
                        if startswith {
                            if let Some(product) = info.product {
                                if product.starts_with(device) {
                                    is_device = true;
                                }
                            }
                        } else if let Some(product) = info.product {
                            if product == device {
                                is_device = true;
                            }
                        }
                    }
                }
                _ => {
                    if !is_desc && p.port_name == device {
                        is_device = true;
                    }
                }
            }

            if is_device {
                if remaining == 0 {
                    found = Some(p.port_name);
                    break;
                }
                remaining -= 1;
            }
        }

        let name = found.ok_or_else(|| format!("{} not found", device))?;
        let port = serialport::new(name, baud)
            .timeout(Duration::from_millis(100))
            .open()?;

        Ok(SerialManager { port })
    }
}
