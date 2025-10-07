import time
from collections import deque

from serial_manager import SerialManager, DataType, unpack_data
from can_manager import CanManager, CAN_DLC

ser_manager = SerialManager(
    "usb-can", baud=115200, device_number=0, print_devices=False
)
can_manager = CanManager(ser_manager)

last_time = time.time()
intervals = deque(maxlen=50)  # average over last 50 frames

while True:
    id, data, dlc_idx = can_manager.receive_frame()

    now = time.time()
    dt = now - last_time
    last_time = now
    intervals.append(dt)

    avg_hz = 1.0 / (sum(intervals) / len(intervals))

    print(f"{avg_hz:.1f} Hz")
