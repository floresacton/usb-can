import time

from serial_manager import SerialManager, DataType, pack_data
from can_manager import CanManager, CAN_DLC

ser_manager = SerialManager(
    "usb-can", baud=115200, device_number=0, print_devices=False
)
can_manager = CanManager(ser_manager)

while True:
    types = [DataType.Float, DataType.Float]
    values = [3.1415, 6.44422]

    can_manager.send_frame(
        20,
        pack_data(types, values),
        CAN_DLC.Size8,
    )
    time.sleep(0.1)
