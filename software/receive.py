import time

from serial_manager import SerialManager, DataType, unpack_data
from can_manager import CanManager, CAN_DLC

ser_manager = SerialManager(
    "usb-can", baud=115200, device_number=1, print_devices=False
)
can_manager = CanManager(ser_manager)

types = [DataType.Float, DataType.Float]

while True:
    id, data, dlc_idx = can_manager.receive_frame()

    values = unpack_data(types, data)
    print(f"Rec {id} {values}")
