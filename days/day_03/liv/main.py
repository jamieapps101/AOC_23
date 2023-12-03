from typing import List
import re
import numpy as np

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
    schematic_grid = np.char.array(all_schematic_rows)
    # print(schematic_grid)
    return schematic_grid

def symbol_check(schematic_grid: List[List[str]]):
    max_x_dimension = np.size(schematic_grid, 0)
    # print(f"Max X = {max_x_dimension}")
    max_y_dimension = np.size(schematic_grid, 1)
    # print(f"Max Y = {max_y_dimension}")
    number_tracker = ""
    valid_part_number = 0
    sum_total = 0
    for x_index in range(schematic_grid.shape[0]):
        for y_index in range(schematic_grid.shape[1]):
            value = schematic_grid[x_index, y_index]
            print(x_index, y_index)
            print(value)
            row = schematic_grid[x_index, :]
            column = schematic_grid[:, y_index]
            # print(row)
            # print(column)
            left_view = row[0:y_index]
            if y_index == 0:
                left_view = ""
            else:
                 left_view = row[y_index - 1]
            right_view = row[y_index + 1 : max_y_dimension]
            if y_index == max_y_dimension - 1:
                right_view = ""
            else:
                right_view = row[y_index + 1]
            up_view = column[0:x_index]
            if x_index == 0:
                up_view = ""
            else:
                up_view = column[x_index - 1]
            down_view = column[x_index + 1 : max_x_dimension]
            if x_index == max_x_dimension - 1:
                down_view = ""
            else:
                down_view = column[x_index + 1]
            if x_index == 0 or y_index == 0:
                up_left_view = ""
            else: 
                up_left_view = schematic_grid[x_index - 1,y_index - 1]
            if x_index == 0 or y_index == max_y_dimension - 1:
                up_right_view = ""
            else:
                up_right_view = schematic_grid[x_index - 1,y_index + 1]
            if x_index == max_x_dimension - 1 or y_index == 0:
                down_left_view = ""
            else:
                down_left_view = schematic_grid[x_index + 1,y_index - 1]
            if x_index == max_x_dimension - 1 or y_index == max_y_dimension - 1:
                down_right_view = ""
            else: down_right_view = schematic_grid[x_index + 1,y_index + 1]

            # print(f"Left View = {left_view}")
            # print(f"Right View = {right_view}")
            # print(f"Up View = {up_view}")
            # print(f"Down View = {down_view}")
            # print(f"Up Left View = {up_left_view}")
            # print(f"Up Right View = {up_right_view}")
            # print(f"Down Left View = {down_left_view}")
            # print(f"Down Right View = {down_right_view}")
            if re.match("[0-9]",value) == None:
                print("Non-number character - check current valid_part_number value!")
                if valid_part_number > 0:
                    number_tracker = int(number_tracker)
                    print(f"Valid part number found! {number_tracker}")
                    sum_total += number_tracker
                    print(f"Current Sum Total: {sum_total}")
                    valid_part_number = 0
                    number_tracker = ""
                else:
                    print(f"No valid part number found - clear number tracker")
                    number_tracker = ""
                    continue
            else:
                print("Number character detected")
                number_tracker = number_tracker + value
                print(f"Current number tracker: {number_tracker}")
                if valid_part_number > 0:
                    print("Part of a valid part number - awaiting next non-number character")
                    continue
                else:
                    print("Checking for symbols...")
                    valid_part_number = view_check(left_view) + view_check(right_view) + view_check(up_view) + view_check(down_view) + view_check(down_left_view) + view_check(down_right_view) + view_check(up_left_view) + view_check(up_right_view)
                    print(f"Symbol Checker: {valid_part_number}")
    print(f"Sum total of valid part numbers: {sum_total}")
    return sum_total
            
def view_check(view: str):
    view_flag = 0
    if view == "":
        view_flag = 0
    elif view == ".":
        view_flag = 0
    elif re.match("[0-9]",view) == None:
        view_flag = 1
    return view_flag

# for each value in the grid:
    # first determine if it is a number - if yes, concat in number tracker variable, if no continue
    # then check the values around it for symbols other than .
    # if present, set part number to true.
    # if the next value on x is a number, continue
    # if the next value on x is a symbol:
        # if part number is false, set number tracker variable to empty and continue
        # if part number is true, cast number tracker variable into an int and add to sum total variable
            # set part number back to false
            # set number tracker back to empty

# once valid part number is set to 1, it needs to stay as 1 until the next detected . character.


def main():
    input_data = read_file(FILE_PATH)
    # print(input_data)
    part_number_sum = symbol_check(input_data)
    print(f"Part number sum total: {part_number_sum}")


if __name__ == "__main__":
    main()