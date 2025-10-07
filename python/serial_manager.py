"""Manages serial connections to devices"""

import struct
from enum import Enum

import serial
import serial.tools.list_ports


class DataType(Enum):
    Empty = 0
    UInt8 = 1
    Int8 = 2
    UInt16 = 3
    Int16 = 4
    UInt32 = 5
    Int32 = 6
    UInt64 = 7
    Int64 = 8
    Float = 9
    Double = 10
    UInt24 = 11


data_lengths = [0, 1, 1, 2, 2, 4, 4, 8, 8, 4, 8, 3]

data_strings = ["", "B", "b", "H", "h", "I", "i", "Q", "q", "f", "d", ""]


def pack_data(types, values):
    packed = b""
    for t, v in zip(types, values):
        if t == DataType.UInt24:
            packed += v.to_bytes(3, "little", signed=False)
        else:
            fmt = "=" + data_strings[t.value]
            packed += struct.pack(fmt, v)
    return packed


def unpack_data(types, data):
    values = []
    offset = 0
    for t in types:
        if t == DataType.UInt24:
            raw = data[offset : offset + 3]
            values.append(int.from_bytes(raw, "little", signed=False))
            offset += 3
        else:
            fmt = "=" + data_strings[t.value]
            size = data_lengths[t.value]
            values.append(struct.unpack(fmt, data[offset : offset + size])[0])
            offset += size
    return tuple(values)


def old_pack_data(types, values):
    fmt = "="
    for type in types:
        fmt += data_strings[type.value]

    return struct.pack(fmt, *values)


def old_unpack_data(types, data):
    fmt = "="
    for type in types:
        fmt += data_strings[type.value]

    return struct.unpack(fmt, data)


class DeviceNotFound(Exception):
    """Cannot find device"""


class SerialManager:
    """Wrapper class for pyserial"""

    def __init__(
        self,
        device,
        baud=115200,
        device_number=0,
        is_desc=True,
        startswith=False,
        print_devices=False,
    ) -> None:
        self.baud = baud
        self.device = None
        ports = serial.tools.list_ports.comports()
        for port in ports:
            if print_devices:
                print(f"Device: {port.device} : {port.description}")
            if is_desc:
                is_device = port.description == device
                if startswith:
                    is_device = port.description.startswith(device)
            else:
                is_device = port.device == device
            if is_device:
                if device_number == 0:
                    self.device = port.device
                    if not print_devices:
                        break

                device_number -= 1

        if self.device:
            self.ser = serial.Serial(self.device, self.baud)
        else:
            raise DeviceNotFound(f"{device} not found")

    def write_bytes(self, bytes):
        """Writes byte array"""
        self.ser.write(bytes)

    def read_bytes(self, length):
        """Waits for length bytes recieved"""
        return self.ser.read(length)

    def read_until(self, sequence):
        """Reads until specific sequence"""
        return self.ser.read_until(expected=sequence)

    def read_line(self):
        """Read line from serial"""
        return self.ser.readline()
