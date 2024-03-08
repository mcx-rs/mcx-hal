import sys
from dataclasses import dataclass

import pandas

if len(sys.argv) != 2:
    exit(1)
file_path = sys.argv[1]

USE_COLS = [
    "ALT0",
    "ALT1",
    "ALT2",
    "ALT3",
    "ALT4",
    "ALT5",
    "ALT6",
    "ALT7",
    "ALT8",
    "ALT9",
    "ALT10",
    "ALT11",
]


def sortkey(x: str):
    x = x.removeprefix("P").split("_")
    port = int(x[0])
    pin = int(x[1])
    return port * 100 + pin


df = pandas.read_excel(file_path, usecols=USE_COLS)
df = df.dropna(subset=["ALT0"])
df = df.sort_values(by="ALT0", key=lambda x: x.map(sortkey))


class Pin:
    port: int
    pin: int
    alt: list[int]

    def __init__(self, port, pin, alt) -> None:
        self.port = port
        self.pin = pin
        self.alt = alt

    def __repr__(self) -> str:
        return f"{self.port}_{self.pin}: {self.alt}"

    def __str__(self) -> str:
        return f"{self.pin}: {self.alt}, Input<Floating>"


ports = {}

for _index, row in df.iterrows():
    x = row["ALT0"].removeprefix("P").split("_")
    port = int(x[0])
    pin = int(x[1])
    alt = []
    for i in range(0, 12):
        if not pandas.isna(row[f"ALT{i}"]):
            alt.append(i)
    p = Pin(port, pin, alt)
    if not port in ports.keys():
        ports[port] = []
    ports[port].append(p)


## Generating GPIO and ALT
for port in ports:
    print(f"gpio!({port}, {port}, [")
    # print(f"    ")
    for pin in ports[port]:
        print(f"    {str(pin)}")
    print(f"]);")

print()
print()

## Generating LPUART ALT
print(f"lpuart!(")
print(f"    [LPUART0]")

# import sys
# import re
# from xml.etree import ElementTree as ET

# if len(sys.argv) != 2:
#     exit(1)
# xml_file_path = sys.argv[1]

# tree = ET.parse(xml_file_path)
# root = tree.getroot()

# pins = root.findall("./pins/pin")


# class PinData:
#     port: int
#     id: int
#     mux: list

#     def __init__(self, port, id, mux) -> None:
#         self.port = port
#         self.id = id
#         self.mux = mux


# regex = r"PIO(\d)_(\d*)\/"
# ports = {}
# for pin in pins:
#     matches = re.findall(regex, pin.attrib["name"])
#     if len(matches) != 1:
#         continue
#     port = matches[0][0]
#     pin_id = matches[0][1]
#     if not port in ports:
#         ports[port] = []

#     mux = set()
#     connections = pin.findall("./connections")
#     for conn in connections:
#         mux.add(conn.attrib["package_function"])
#     mux = [int(m.removeprefix("alt")) for m in mux]
#     mux.sort()
#     ports[port].append(PinData(int(port), int(pin_id), mux))

# for port in ports.keys():
#     ports[port].sort(key=lambda x: x.id)
# print(ports)
# ports = [(port, pins) for port, pins in ports.items()]
# ports.sort(key=lambda x: x[0])

# for port, pins in ports:
#     print(f"gpio!({port}, {port}, [")
#     for pin in pins:
#         print(f"    {pin.id}: [", end="")
#         print(*pin.mux, sep=", ", end="")
#         print("], Input<Floating>,")
#     print("]);")
