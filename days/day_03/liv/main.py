from typing import List, Optional
import re
import json

FILE_PATH = "days/day_03/liv/input_data.txt"

def read_file(file_path: str) -> List[str]:
    schematic_row = []
    all_schematic_rows = []
    with open(file_path) as fp:
        for line in fp:
            line = line.rstrip()
            for char in line:
                schematic_row.append(char)
                # print(schematic_row)
            all_schematic_rows.append(schematic_row)
            schematic_row = []
    # print(all_schematic_rows)
    return all_schematic_rows

def pretty_print(map: List[List[str]]):
    for line in map:
        print("".join(line))

def generate_empty_star_map(string_length: int, list_length: int) -> List[List[str]]:
    empty_map = []
    for row_index in range(list_length):
        pad_character = []
        for character_index in range(string_length):
            pad_character.append(".")
        # pad_character = "."*string_length
        empty_map.append(pad_character)
    return empty_map

def star_isolation(input_data: List[str]) -> List[List[List[str]]]:
    isolated_star_list = []
    for row_index in range(len(input_data)):
        for item_index_in_row in range(len(input_data[row_index])):
            if input_data[row_index][item_index_in_row] == "*":
                temp_map = generate_empty_star_map(len(input_data[0]),len(input_data))
                temp_map[row_index][item_index_in_row] = "*"
                isolated_star_list.append(temp_map)
    # print(isolated_star_list)            
    return isolated_star_list

def number_isolation(input_data: List[str]) -> List[List[str]]:
    temp_map = generate_empty_star_map(len(input_data[0]),len(input_data))
    for row_index in range(len(input_data)):
        for item_index_in_row in range(len(input_data[row_index])):
            if input_data[row_index][item_index_in_row] in "1234567890":                
                temp_map[row_index][item_index_in_row] = input_data[row_index][item_index_in_row]
    return temp_map

def pad_found_stars(input_data: List[List[str]]) -> List[List[str]]:
    padded_map = generate_empty_star_map(len(input_data[0]),len(input_data))
    for row_index in range(len(input_data)):
        for character_index in range(len(input_data[row_index])):
            if input_data[row_index][character_index] == ".":
                continue
            for row_index_increment in [-1,0,1]:
                for character_index_increment in [-1,0,1]:
                    row_index_offset = row_index+row_index_increment
                    character_index_offset = character_index+character_index_increment
                    if row_index_offset < 0 or character_index_offset < 0 or row_index_offset == len(input_data) or character_index_offset == len(input_data[row_index_offset]):
                        continue
                    padded_map[row_index_offset][character_index_offset] = "*"
    # print(json.dumps(padded_map,indent=4))
    return padded_map

def compare_data(reference_data: List[List[str]],padded_map: List[List[str]]) -> List[List[str]]:
    number_construction_map = generate_empty_star_map(len(padded_map[0]),len(padded_map))
    for row_index in range(len(padded_map)):
        for character_index in range(len(padded_map[row_index])):
            if padded_map[row_index][character_index] == "*":
                number_construction_map[row_index][character_index] = reference_data[row_index][character_index]
    return number_construction_map

def clumpy_state_machine(reference_data: List[List[str]], number_construction_map: [List[List[str]]]) -> Optional[int]:
    clumps_in_given_map = []
    list_len = len(number_construction_map)
    sub_list_len = len(number_construction_map[0])
    clean_map = generate_empty_star_map(sub_list_len,list_len)
    for row_index in range(len(number_construction_map)):
        column_index = 0
        ref_data_column_index = 0
        clump_start = None
        clump_end = None
        clump_state = 0
        iteration_counter = 0
        while True:
            iteration_counter += 1
            if iteration_counter == 1000:
                import sys
                print("Max iterations hit")
                sys.exit(1)
            # print(f"Current state is: {clump_state}")
            if clump_state == 0:
                if column_index >= len(number_construction_map[row_index]):
                    break
                # print(f"row_index {row_index} column_index {column_index}")
                if number_construction_map[row_index][column_index] != ".":
                    clump_state += 1
                    clump_start = column_index
                column_index += 1
            elif clump_state == 1:
                if number_construction_map[row_index][column_index] == ".":
                    clump_end = column_index - 1
                    clump_state += 1
                    ref_data_column_index = clump_start
                column_index += 1
                # print(f"column_index: {column_index}")
            elif clump_state == 2:
                # print('clump state 2')
                ref_data_column_index -= 1
                if ref_data_column_index < 0 or reference_data[row_index][ref_data_column_index] == ".":
                    clump_start = ref_data_column_index + 1
                    clump_state += 1
                    ref_data_column_index = clump_end
            elif clump_state == 3:
                # print('clump state 3')
                ref_data_column_index += 1
                if ref_data_column_index == len(number_construction_map[row_index]) or reference_data[row_index][ref_data_column_index] == '.':
                    # print('we at clump end {} {}'.format(ref_data_column_index == len(number_construction_map[row_index]), number_construction_map[row_index][ref_data_column_index] == '.'))
                    # print(f'ref_data_column_index: {ref_data_column_index}')
                    clump_end = ref_data_column_index
                    clump_state += 1
                    column_index = ref_data_column_index
            elif clump_state == 4:
                # print('clump state 4')
                clump_string = reference_data[row_index][clump_start:clump_end]
                clumps_in_given_map.append({
                    "clump_start": clump_start,
                    "clump_end": clump_end,
                    "value": int(''.join(clump_string))
                })
                clump_state = 0
                # build a clean map
                clean_map[row_index][clump_start:clump_end-1] = clump_string
    # print("----------------")
    # pretty_print(clean_map)
    if len(clumps_in_given_map) == 2:
        return clumps_in_given_map[0]["value"] * clumps_in_given_map[1]["value"]


def main():
    input_data = read_file(FILE_PATH)
    print(len(input_data))
    isolated_star_maps = star_isolation(input_data)
    isolated_number_maps = number_isolation(input_data)
    # pretty_print(isolated_number_maps)
    gear_ratio_total = 0
    for map in isolated_star_maps:
        # print("================")
        # pretty_print(map)
        padded_map = pad_found_stars(map)
        # print("----------------")
        # pretty_print(input_data)
        # pretty_print(padded_map)
        number_construction_map = compare_data(isolated_number_maps,padded_map)
        # print("----------------")
        # pretty_print(number_construction_map)
        gear_ratio = clumpy_state_machine(isolated_number_maps,number_construction_map)
        if gear_ratio is not None:
            # print(f"Gear ratio: {gear_ratio}")
            gear_ratio_total += gear_ratio
    print(f"Gear ratio total:{gear_ratio_total}")

if __name__ == "__main__":
    main()