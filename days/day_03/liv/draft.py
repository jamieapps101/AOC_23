from typing import List, Tuple
import re
import numpy as np

FILE_PATH = "days/day_03/liv/test_data.txt"

def read_file(file_path: str) -> Tuple[np.char.array,np.char.array]:
    schematic_row = []
    all_schematic_rows = []
    number_row = []
    all_number_rows = []
    gear_row = []
    all_gear_rows = []
    with open(file_path) as fp:
        for line in fp:
            line = line.rstrip()
            for char in line:
                schematic_row.append(char)
                # print(schematic_row)
                if re.match("[0-9]",char) != None:
                    number_row.append(char)
                else:
                    number_row.append(".")
                if re.match("\*",char) != None:
                    gear_row.append(char)
                else:
                    gear_row.append(".")
            all_schematic_rows.append(schematic_row)
            all_number_rows.append(number_row)
            all_gear_rows.append(gear_row)
            schematic_row = []
            number_row = []
            gear_row = []
    # print(all_schematic_rows)
    print(all_number_rows)
    print(all_gear_rows)
    schematic_grid = np.char.array(all_schematic_rows)
    number_grid = np.char.array(all_number_rows)
    gear_grid = np.char.array(all_gear_rows)
    # print(schematic_grid)
    # print(number_grid)
    # print(gear_grid)
    return (number_grid, gear_grid)

def grid_compare(input_data: Tuple[np.char.array,np.char.array]):
    (number_grid, gear_grid) = input_data
    print(number_grid)
    print(gear_grid)
    

def main():
    input_data = read_file(FILE_PATH)
    # print(input_data)
    grids = grid_compare(input_data)

if __name__ == "__main__":
    main()

# find stars - find elements of a certain value
# for loop:
#   new grid with one star in it
#   dilation - for a star, populate eight stars around it
#   numpy logical and (??) take numbers in the numbers frame and logical and them with stars