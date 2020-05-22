import xml.etree.ElementTree as ElementTree
import re
from pathlib import Path

from natsort import natsorted

dir_path = Path(r'C:\Program Files (x86)\STMicroelectronics\STM32Cube\STM32CubeMX\db\mcu\IP')
FEATURE = 'F0'

p = '{http://mcd.rou.st.com/modules.php?name=mcu}GPIO_Pin'
s = '{http://mcd.rou.st.com/modules.php?name=mcu}PinSignal'
sp = '{http://mcd.rou.st.com/modules.php?name=mcu}SpecificParameter'
pv = '{http://mcd.rou.st.com/modules.php?name=mcu}PossibleValue'

re_chip_name = re.compile(r'^GPIO-STM32(\w*)_gpio\w*.xml$')
re_af_name = re.compile(r'^GPIO_(AF\d*)_\w*$')
re_pin_name = re.compile(r'^P[A-K]\d{1,2}')


def print_red(*args):
    print('\033[31m', end='')
    print(*args, end='')
    print('\033[0m')


def transpose(din):
    dout = {}
    for key1, inner in din.items():
        for key2, value in inner.items():
            dout.setdefault(key2, {})[key1] = value
    return dout


def get_pins():
    pins = {}

    for path in dir_path.iterdir():
        if not path.name.startswith('GPIO-STM32' + FEATURE):
            continue
        chip_type = re_chip_name.match(path.name).group(1)

        tree = ElementTree.parse(str(path))
        root = tree.getroot()

        for pin in root.findall(p):
            for signal in pin.findall(s):
                for parameter in signal.findall(sp):
                    for possible_value in parameter.findall(pv):
                        pin_name = re_pin_name.findall(pin.attrib['Name'])[0]
                        signal_name = signal.attrib['Name']
                        try:
                            af = re_af_name.match(possible_value.text).group(1)
                            pins \
                                .setdefault(pin_name, {}) \
                                .setdefault(signal_name, {}) \
                                .setdefault(af, []).append(chip_type)
                        except AttributeError:
                            print_red(path, possible_value.text)

    return pins


def get_afs(pattern: re.Pattern):
    all_pins = get_pins()
    all_signals = transpose(all_pins)

    outstring = ''

    for signal, pins in natsorted(all_signals.items()):
        if not pattern.match(signal):
            continue

        outstring += f'static {signal}: Map<&str, u8> = phf_map! {{\n'
        for pin, afs in natsorted(pins.items()):
            assert len(afs) == 1
            af = list(afs.keys())[0]
            outstring += f'    "{pin.lower()}" => {af[-1]},\n'
        outstring += '};\n\n'

        mapped_signals.append(signal)

    return outstring


if __name__ == '__main__':
    mapped_signals = []

    re_spis = re.compile(r'^SPI\d_(SCK|MISO|MOSI)')
    re_i2cs = re.compile(r'^I2C\d_(SCL|SDA)')
    re_uarts = re.compile(r'^USART\d_(TX|RX)')

    outstring = ''
    outstring += get_afs(re_i2cs)
    outstring += get_afs(re_spis)
    outstring += get_afs(re_uarts)

    af_map_string = 'pub static AF_MAP: Map<&str, &Map<&str, u8>> = phf_map! {\n'
    for signal in mapped_signals:
        af_map_string += f'    "{signal}" => &{signal},\n'
    af_map_string += '};\n\n'

    imports = 'use phf::{phf_map, Map};\n\n'

    print(imports + af_map_string + outstring)
