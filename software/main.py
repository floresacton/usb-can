import struct
import time
from enum import Enum

from serial_manager import SerialManager

manager = SerialManager("usb-can")


class DLC_SIZE(Enum):
    DLC_0 = 0
    DLC_1 = 1
    DLC_2 = 2
    DLC_3 = 3
    DLC_4 = 4
    DLC_5 = 5
    DLC_6 = 6
    DLC_7 = 7
    DLC_8 = 8
    DLC_12 = 9
    DLC_16 = 10
    DLC_20 = 11
    DLC_24 = 12
    DLC_32 = 13
    DLC_48 = 14
    DLC_64 = 15


dlc_sizes = [0, 1, 2, 3, 4, 5, 6, 7, 8, 12, 16, 20, 24, 32, 48, 64]


def send_frame(id, data, dlc_idx):
    header = dlc_idx << 11 | id
    header_bytes = struct.pack("<H", header)
    frame = bytearray()
    frame.extend(header_bytes)
    frame.extend(data)
    manager.write_bytes(frame)


def receive_frame():
    header = struct.unpack("<H", manager.read_bytes(2))[0]
    id = header & 0x07FF
    dlc_idx = header >> 11
    data = manager.read_bytes(dlc_sizes[dlc_idx])
    return id, data, dlc_idx


send_frame(13, bytearray([1, 2, 3, 6, 10]), DLC_SIZE.DLC_5.value)
print(receive_frame())
