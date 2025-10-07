import math
import struct
import time
from enum import Enum

from serial_manager import SerialManager

manager = SerialManager("usb-can")


class Commands(Enum):
    CommandSetP = 0
    CommandSetI = 1
    CommandSetD = 2


class DLC(Enum):
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


# 0  CommandSetP, //float
# 1  CommandSetI, //float
# 2  CommandSetD, //float
# 3  CommandSetMaxP, //float
# 4  CommandSetMaxI, //float
# 5  CommandSetMaxD, //float
# 6  CommandSetMaxOut, //uint8_t
# 7  CommandSetVoltage, //int8_t
# 8  CommandSetCurrent, //uint8_t
# 9  CommandSetTarget, //int32
# 10 CommandSetMode, //uint8_t
# 11 CommandSetAddress,//uint16_t
# 12 CommandReset, //none

# //request commands
# 13 CommandGetPosition,
# 14 CommandGetCurrent,
# 15 CommandGetMode,

# //responses
# 16 CommandSendPosition, //int32_t
# 17 CommandSendCurrent, //uint16_t
# 18 CommandSendMode, //uint8_t


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


# # set mode PID
# data = bytearray([0])
# data.extend(struct.pack("b", 1))
# send_frame(10, data, DLC.Size2.value)

# # set max P
# data = bytearray([0])
# data.extend(struct.pack("f", 100))
# send_frame(3, data, DLC.Size5.value)

# # set P
# data = bytearray([0])
# data.extend(struct.pack("f", 0.1))
# send_frame(0, data, DLC.Size5.value)

# # set max out
# data = bytearray([0])
# data.extend(struct.pack("f", 100))
# send_frame(6, data, DLC.Size5.value)

# # set current
# send_frame(8, bytearray([0, 20]), DLC.Size2.value)

# # reset
# send_frame(12, bytearray([0]), DLC.Size1.value)

# # # set duty
# # data = bytearray([0])
# data.extend(struct.pack("H", 100))
# # send_frame(7, data, DLC.Size2.value)

# # set position
# data = bytearray([0])
# data.extend(struct.pack("i", 10000))
# send_frame(9, data, DLC.Size5.value)

# # set address
# data = struct.pack("H", 100)
# send_frame(11, data, DLC.Size3.value)

# time.sleep(5)

# data = bytearray([0])
# data.extend(struct.pack("i", 0))
# send_frame(109, data, DLC.Size5.value)

########### current test ################

# set mode Manual
data = bytearray([0])
data.extend(struct.pack("b", 0))
send_frame(10, data, DLC.Size2.value)

# set voltage
data = bytearray([0])
data.extend(struct.pack("b", 65))
send_frame(7, data, DLC.Size2.value)

# set current
send_frame(8, bytearray([0, 20]), DLC.Size2.value)

#  print current
while True:
    send_frame(14, bytearray([0]), DLC.Size1.value)
    frame = receive_frame()
    id, data, dlc_index = frame
    print(struct.unpack("<h", data)[0])
    time.sleep(0.1)

#######################

# start = time.time()

# while True:
#     seconds = time.time() - start
#     duty = int(15 * math.sin(seconds))
#     if duty < 0:
#         duty -= 43
#     else:
#         duty += 43

#     # set duty
#     data = bytearray([0])
#     data.extend(struct.pack("b", duty))
#     send_frame(7, data, DLC.Size2.value)

#     # get position
#     send_frame(11, bytearray([0]), DLC.Size1.value)
#     frame = receive_frame()
#     id, data, dlc_index = frame
#     print(struct.unpack("<i", data)[0])
#     time.sleep(0.1)
