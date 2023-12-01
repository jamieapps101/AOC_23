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

def number_cruncher(input_data: List[str]) -> int:
    totaliser = 0
    for number in input_data:
        print(number)
        numbers_to_crunch = re.findall("[0-9]",number)
        #print(numbers_to_crunch)
        length = len(numbers_to_crunch)
        #print(length)
        if length == 1:
            print("One int only")
            first_value = numbers_to_crunch[0]
            last_value = numbers_to_crunch[0]
            print(first_value)
            print(last_value)
            final_value = int(first_value + last_value)
            print(final_value)
            totaliser = totaliser + final_value
            print(totaliser)
        else:
            print("More than one int")
            first_value = numbers_to_crunch[0]
            last_value = numbers_to_crunch[length - 1]
            print(first_value)
            print(last_value)
            final_value = int(first_value + last_value)
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