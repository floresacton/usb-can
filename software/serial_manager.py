"""Manages serial connections to devices"""

import serial
import serial.tools.list_ports


class DeviceNotFound(Exception):
    """Cannot find device"""


class SerialManager:
    """Wrapper class for pyserial"""

    def __init__(self, device_name, baud=115200, device_number=0) -> None:
        self.baud = baud
        ports = serial.tools.list_ports.comports()
        for port in ports:
            if port.description == device_name:
                if device_number == 0:
                    self.device = port.device
                    self.ser = serial.Serial(self.device, self.baud)
                    return

                device_number -= 1

        raise DeviceNotFound(f"{device_name} not found")

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
