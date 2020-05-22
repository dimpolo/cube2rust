import xml.etree.ElementTree as ElementTree

from natsort import natsorted

file_path = r'C:\Program Files (x86)\STMicroelectronics\STM32Cube\STM32CubeMX\db\mcu\families.xml'
FEATURE = 'F0'


def get_memory_sizes():
    mem_info = {}

    tree = ElementTree.parse(file_path)
    root = tree.getroot()

    for family in root:
        for subfamily in family:
            for mcu in subfamily:
                mcu_name = mcu.attrib['RefName']
                if not mcu_name.startswith('STM32' + FEATURE):
                    continue

                rams = mcu.findall('Ram')
                flashs = mcu.findall('Flash')

                assert len(rams) == 1
                assert len(flashs) == 1

                flash = flashs[0].text
                ram = rams[0].text

                assert mcu_name not in mem_info
                mem_info[mcu_name] = {'ram': ram, 'flash': flash}

    return mem_info


def get_mem_sizes_string(mem_info: dict):
    outstring = 'pub static MEMORY_SIZES: Map<&str, MemSize> = phf_map! {\n'

    for mcu, mem_size in natsorted(mem_info.items()):
        flash = mem_size['flash']
        ram = mem_size['ram']
        outstring += f'    "{mcu}" => MemSize{{flash: {flash}, ram: {ram}}},\n'

    return outstring + '};\n'


if __name__ == '__main__':
    mem_info = get_memory_sizes()
    mem_sizes_string = get_mem_sizes_string(mem_info)

    imports = 'use phf::{phf_map, Map};\n'\
              'use super::MemSize;\n\n'

    print(imports + mem_sizes_string)
