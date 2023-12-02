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

def blue_dice_check(input_data: str):
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
    highest_blue_dice = blue_dice_counts[len(blue_dice_counts)- 1]
    print(highest_blue_dice)
    return highest_blue_dice
    

def red_dice_check(input_data: str):
    # input_data = "Game 1: 5 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
    game_number = int(re.search(r'\d{1,100}',input_data).group()) 
    print(game_number)

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
    highest_red_dice = red_dice_counts[len(red_dice_counts)- 1]
    print(highest_red_dice)
    return(highest_red_dice)
    

def green_dice_check(input_data: str):
    # input_data = "Game 1: 5 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
    game_number = int(re.search(r'\d{1,100}',input_data).group()) 
    print(game_number)

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
    highest_green_dice = green_dice_counts[len(green_dice_counts)- 1]
    print(highest_green_dice)
    return(highest_green_dice)

# Sorted list of integers for each game type - look at last item in the list - if higher than, game not possible. If lower, store TRUE and go to next colour.

# The Elf would first like to know which games would have been possible if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes?

# Counter for each game - blue, red green
# If all individual values are below the stated values for that colour, log the number of the game
# If they are above the stated values, continue

def main():
    input_data = read_file(FILE_PATH)
    print(input_data)
    game_cube_power = 0
    total_cube_power = 0
    for game in input_data:
        highest_blue_dice = blue_dice_check(game)
        print(highest_blue_dice)
        highest_red_dice = red_dice_check(game)
        print(highest_red_dice)
        highest_green_dice = green_dice_check(game)
        print(highest_green_dice)
        game_cube_power = highest_blue_dice*highest_red_dice*highest_green_dice
        print(game_cube_power)
        total_cube_power += game_cube_power
        print(total_cube_power)
    print(total_cube_power)

if __name__ == "__main__":
    main()