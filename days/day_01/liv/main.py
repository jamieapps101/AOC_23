from typing import List
import re

FILE_PATH = "days/day_01/liv/input_data.txt"

def read_file(file_path: str) -> List[str]:
    lines = []
    with open(file_path) as fp:
        for line in fp:
            if line == "":
                continue
            else:
                line = line.rstrip()
                lines.append(line)
    #print(lines)
    return lines

def reverse_string(input_data: str):
    return input_data[::-1]

def find_first_number(input_data: str) -> str:
    find_first_number = re.findall("([0-9]|one|two|three|four|five|six|seven|eight|nine)",input_data)[0]
    print(find_first_number)
    if re.search("[0-9]",find_first_number):
        first_number = find_first_number
        # print(first_number)
    elif find_first_number == "one":
        first_number = "1"
    elif find_first_number == "two":
        first_number = "2"
    elif find_first_number == "three":
        first_number = "3"
    elif find_first_number == "four":
        first_number = "4"
    elif find_first_number == "five":
        first_number = "5"
    elif find_first_number == "six":
        first_number = "6"
    elif find_first_number == "seven":
        first_number = "7"
    elif find_first_number == "eight":
        first_number = "8"
    elif find_first_number == "nine":
        first_number = "9"
    return first_number

def find_last_number(input_data: str) -> str:
    input_data = reverse_string(input_data)
    find_last_number = re.findall("([0-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)",input_data)[0]
    # print(find_last_number)
    if re.search("[0-9]",find_last_number):
        last_number = find_last_number
        # print(last_number)
    elif find_last_number == "eno":
        last_number = "1"
    elif find_last_number == "owt":
        last_number = "2"
    elif find_last_number == "eerht":
        last_number = "3"
    elif find_last_number == "ruof":
        last_number = "4"
    elif find_last_number == "evif":
        last_number = "5"
    elif find_last_number == "xis":
        last_number = "6"
    elif find_last_number == "neves":
        last_number = "7"
    elif find_last_number == "thgie":
        last_number = "8"
    elif find_last_number == "enin":
        last_number = "9"
    return last_number

def number_cruncher(input_data: List[str]) -> int:
    first_number = ""
    last_number = ""
    totaliser = 0
    for number in input_data:
        print(number)
        first_number = find_first_number(number)
        last_number = find_last_number(number)
        final_value = first_number + last_number
        final_value = int(final_value)
        print(final_value)
        totaliser = totaliser + final_value
        print(totaliser)
    return totaliser

def main():
    input_data = read_file(FILE_PATH)
    #print(input_data)
    output_data = number_cruncher(input_data)
    print(output_data)

if __name__ == "__main__":
    main()