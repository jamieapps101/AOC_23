from typing import List
import re

FILE_PATH = "days/day_02/liv/input_data.txt"

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

def game_checker(input_data: str):
    # input_data = "Game 1: 5 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
    game_number = int(re.search(r'\d{1,100}',input_data).group()) 
    print(game_number)
    
    blue_games = re.findall("[0-9]{1,5} blue",input_data)
    # print(blue_games)
    blue_dice_counts = []
    for show_of_hand in blue_games:
        number_of_dice = int(re.search('\d{1,5}',show_of_hand).group())
        # print(number_of_dice)
        blue_dice_counts.append(number_of_dice)
        # print(blue_dice_counts)
    # print(blue_dice_counts)
    blue_dice_counts = sorted(blue_dice_counts)
    # print(blue_dice_counts[len(blue_dice_counts)- 1])
    if blue_dice_counts[len(blue_dice_counts)- 1] > 14:
        blue_valid = False
        # print(blue_valid)
        print("Too many blue!")
    else:
        blue_valid = True
        # print(blue_valid)
        print("Blue - valid game")

    red_games = re.findall("[0-9]{1,5} red",input_data)
    # print(red_games)
    red_dice_counts = []
    for show_of_hand in red_games:
        number_of_dice = int(re.search('\d{1,5}',show_of_hand).group())
        # print(number_of_dice)
        red_dice_counts.append(number_of_dice)
        # print(red_dice_counts)
    # print(red_dice_counts)
    red_dice_counts = sorted(red_dice_counts)
    # print(red_dice_counts[len(red_dice_counts)- 1])
    if red_dice_counts[len(red_dice_counts)- 1] > 12:
        red_valid = False
        # print(red_valid)
        print("Too many red!")
    else:
        red_valid = True
        # print(red_valid)
        print("Red - valid game")

    green_games = re.findall("[0-9]{1,5} green",input_data)
    # print(green_games)
    green_dice_counts = []
    for show_of_hand in green_games:
        number_of_dice = int(re.search('\d{1,5}',show_of_hand).group())
        # print(number_of_dice)
        green_dice_counts.append(number_of_dice)
        # print(green_dice_counts)
    # print(green_dice_counts)
    green_dice_counts = sorted(green_dice_counts)
    # print(green_dice_counts[len(green_dice_counts)- 1])
    if green_dice_counts[len(green_dice_counts)- 1] > 13:
        green_valid = False
        # print(green_valid)
        print("Too many green!")
    else:
        green_valid = True
        # print(green_valid)
        print("Green - valid game")
    
    if blue_valid == True and red_valid == True and green_valid == True:
        return game_number
    else:
        print("Invalid game")

# Sorted list of integers for each game type - look at last item in the list - if higher than, game not possible. If lower, store TRUE and go to next colour.

# The Elf would first like to know which games would have been possible if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes?

# Counter for each game - blue, red green
# If all individual values are below the stated values for that colour, log the number of the game
# If they are above the stated values, continue

def main():
    input_data = read_file(FILE_PATH)
    print(input_data)
    valid_game_sum = 0
    for game in input_data:
        checker_output = game_checker(game)
        print(checker_output)
        if checker_output == None:
            print("Invalid game - no addition triggered")
            continue
        else:
            valid_game_sum += checker_output
            print("Addition triggered")
            print(valid_game_sum)
    print(valid_game_sum)


if __name__ == "__main__":
    main()