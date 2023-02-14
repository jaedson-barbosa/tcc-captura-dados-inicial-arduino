import serial
import time
from datetime import datetime

com = "COM4"
baud = 115200
timeformat = "%d/%m/%Y %H:%M:%S"

x = serial.Serial(com, baud, timeout = 0.1)

with open('result.txt', 'a') as file:
    while x.isOpen() == True:
        data = str(x.readline().decode('utf-8')).rstrip()
        time = datetime.now().strftime(timeformat)
        data = '"' + time + "\"," + data
        print(data)
        file.write(data + '\n')