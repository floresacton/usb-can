import struct
from enum import Enum


class CAN_DLC(Enum):
    Size0 = 0
    Size1 = 1
    Size2 = 2
    Size3 = 3
    Size4 = 4
    Size5 = 5
    Size6 = 6
    Size7 = 7
    Size8 = 8
    Size12 = 9
    Size16 = 10
    Size20 = 11
    Size24 = 12
    Size32 = 13
    Size48 = 14
    Size64 = 15

dlc_sizes = [0, 1, 2, 3, 4, 5, 6, 7, 8, 12, 16, 20, 24, 32, 48, 64]


class CAN_TYPE(Enum):
    EMPTY = 0
    UINT8_T = 1
    INT8_T = 2
    UINT16_T = 3
    INT16_T = 4
    UINT32_T = 5
    INT32_T = 6
    Float = 7

data_lengths = [0, 1, 1, 2, 2, 4, 4, 4]

data_strings = ["", "B", "b", "H", "h", "I", "i", "f"]

# TODO debug
class CanManager:
    def __init__(self, manager) -> None:
        self.manager = manager
    
    def send_frame(self, id, data, length):
        header = length.value << 11 | id
        bytes = struct.pack("H", header)
        bytes.extend(data)
        self.manager.write_bytes(bytes)

    def receive_frame(self):
        header = struct.unpack("H", self.manager.read_bytes(2))
        id = header & 0x07FF
        dlc_idx = header >> 11
        data = self.manager.read_bytes(dlc_sizes[dlc_idx])
        return id, data, dlc_idx

# TODO finish
    def send_message(self, types, values):
        parse_string = "".join(map(lambda type: data_strings(type.value), types))
        data = struct.pack(parse_string, values)
        self.manager.write_bytes(data)
            
# TODO finish
    def parse_message(self, types, data):
        ...