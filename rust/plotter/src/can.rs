use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::error::Error;

use crate::serial::SerialManager;

/// DLC (Data Length Code)
#[derive(Debug, Clone, Copy)]
pub enum CanDlc {
    Size0 = 0,
    Size1 = 1,
    Size2 = 2,
    Size3 = 3,
    Size4 = 4,
    Size5 = 5,
    Size6 = 6,
    Size7 = 7,
    Size8 = 8,
    Size12 = 9,
    Size16 = 10,
    Size20 = 11,
    Size24 = 12,
    Size32 = 13,
    Size48 = 14,
    Size64 = 15,
}

impl CanDlc {
    pub fn size(self) -> usize {
        match self {
            CanDlc::Size0 => 0,
            CanDlc::Size1 => 1,
            CanDlc::Size2 => 2,
            CanDlc::Size3 => 3,
            CanDlc::Size4 => 4,
            CanDlc::Size5 => 5,
            CanDlc::Size6 => 6,
            CanDlc::Size7 => 7,
            CanDlc::Size8 => 8,
            CanDlc::Size12 => 12,
            CanDlc::Size16 => 16,
            CanDlc::Size20 => 20,
            CanDlc::Size24 => 24,
            CanDlc::Size32 => 32,
            CanDlc::Size48 => 48,
            CanDlc::Size64 => 64,
        }
    }
}

pub struct CanFrame {
    pub id: u16,
    pub dlc: CanDlc,
    pub data: Vec<u8>,
}

/// CAN manager over a `SerialManager`
pub struct CanManager<'a> {
    manager: &'a mut SerialManager,
}

impl<'a> CanManager<'a> {
    pub fn new(manager: &'a mut SerialManager) -> Self {
        CanManager { manager }
    }

    pub fn send_frame(
        &mut self,
        can_id: u16,
        data: &[u8],
        dlc: CanDlc,
    ) -> Result<(), Box<dyn Error>> {
        let header: u16 = ((dlc as u16) << 11) | (can_id & 0x07FF);

        let mut buf = Vec::with_capacity(2 + data.len());
        buf.write_u16::<LittleEndian>(header)?;
        buf.extend_from_slice(data);

        self.manager.port.write_all(&buf)?;
        Ok(())
    }

    pub fn receive_frame(&mut self) -> Result<CanFrame, Box<dyn Error>> {
        // read header
        let mut header_buf = [0u8; 2];
        self.manager.port.read_exact(&mut header_buf)?;
        let header = (&header_buf[..]).read_u16::<LittleEndian>()?;

        let can_id = header & 0x07FF;
        let dlc_idx = header >> 11;

        let dlc = match dlc_idx {
            0 => CanDlc::Size0,
            1 => CanDlc::Size1,
            2 => CanDlc::Size2,
            3 => CanDlc::Size3,
            4 => CanDlc::Size4,
            5 => CanDlc::Size5,
            6 => CanDlc::Size6,
            7 => CanDlc::Size7,
            8 => CanDlc::Size8,
            9 => CanDlc::Size12,
            10 => CanDlc::Size16,
            11 => CanDlc::Size20,
            12 => CanDlc::Size24,
            13 => CanDlc::Size32,
            14 => CanDlc::Size48,
            15 => CanDlc::Size64,
            _ => return Err("Invalid DLC index".into()),
        };

        let mut data = vec![0u8; dlc.size()];
        self.manager.port.read_exact(&mut data)?;

        Ok(CanFrame {
            id: can_id,
            dlc,
            data,
        })
    }
}
