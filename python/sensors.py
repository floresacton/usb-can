import time
from collections import deque

from serial_manager import SerialManager, DataType, unpack_data
from can_manager import CanManager, CAN_DLC

ser_manager = SerialManager(
    "usb-can", baud=115200, device_number=0, print_devices=False
)
can_manager = CanManager(ser_manager)

types = [
    DataType.UInt16,
    DataType.UInt24,
    DataType.UInt24,
    DataType.UInt24,
    DataType.UInt24,
    DataType.UInt24,
    DataType.UInt24,
    DataType.Int16,
    DataType.Int16,
    DataType.Int16,
    DataType.Int16,
    DataType.Int16,
    DataType.Int16,
]

last_time = time.time()
intervals = deque(maxlen=50)  # average over last 50 frames

while True:
    id, data, dlc_idx = can_manager.receive_frame()

    now = time.time()
    dt = now - last_time
    last_time = now
    intervals.append(dt)

    avg_hz = 1.0 / (sum(intervals) / len(intervals))

    values = unpack_data(types, data)
    pressures = [val / 40960 for val in values[0:7]]
    accel = [val / 8192 for val in values[7:10]]
    gyro = [val / 65.5 for val in values[10:13]]

    if id == 1:
        print(f"Recieved from {id}")
        print(f"Pressure: {[f'{p:6.3f}' for p in pressures]}")
        print(f"Accel:    {[f'{a:6.2f}' for a in accel]}")
        print(f"Gyro:     {[f'{g:6.1f}' for g in gyro]}")

        print(f"{avg_hz:.1f} Hz")
