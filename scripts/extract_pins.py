import sys
import re
from xml.etree import ElementTree as ET

if len(sys.argv) != 2:
    exit(1)
xml_file_path = sys.argv[1]

tree = ET.parse(xml_file_path)
root = tree.getroot()

pins = root.findall("./pins/pin")


class PinData:
    port: int
    id: int
    mux: list

    def __init__(self, port, id, mux) -> None:
        self.port = port
        self.id = id
        self.mux = mux


regex = r"PIO(\d)_(\d*)\/"
ports = {}
for pin in pins:
    matches = re.findall(regex, pin.attrib["name"])
    if len(matches) != 1:
        continue
    port = matches[0][0]
    pin_id = matches[0][1]
    if not port in ports:
        ports[port] = []

    mux = set()
    connections = pin.findall("./connections")
    for conn in connections:
        mux.add(conn.attrib["package_function"])
    mux = [int(m.removeprefix("alt")) for m in mux]
    mux.sort()
    ports[port].append(PinData(int(port), int(pin_id), mux))

for port in ports.keys():
    ports[port].sort(key=lambda x: x.id)
print(ports)
ports = [(port, pins) for port, pins in ports.items()]
ports.sort(key=lambda x: x[0])

for port, pins in ports:
    print(f"gpio!({port}, {port}, [")
    for pin in pins:
        print(f"    {pin.id}: [", end="")
        print(*pin.mux, sep=", ", end="")
        print("], Input<Floating>,")
    print("]);")
