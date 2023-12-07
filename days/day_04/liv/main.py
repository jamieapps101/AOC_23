from typing import List
import re

FILE_PATH = "days/day_04/liv/input_data.txt"

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

def point_calculator(input_data: List[str]) -> int:
    card_number = 0
    character_count = 0
    summed_card_scores = []
    for row in input_data:
        card_number = int(re.search("[0-9]{1,100}",row).group())
        print(f"Card number is {card_number}")
        row = re.sub("  "," 0",row)
        print(row)
        row = re.sub("Card [0-9]{1,1000}: ","",row)
        print(row)
        split_string = row.split("|")
        print(f"Split string is {split_string}")
        winning_number_list = split_string[0].lstrip()
        scratchcard_number_list = split_string[1].rstrip()
        print(f"Winning numbers are: {winning_number_list}")
        print(f"Scratchcard numbers are: {scratchcard_number_list}")
        scratchcard_number = ""
        win_count = 0
        card_score = 0
        for char in scratchcard_number_list:
            line_length = len(scratchcard_number_list)
            # print(f"Line length: {line_length}")
            # print(f"Current char: {char}")
            print(f"Character count: {character_count}")
            if character_count == line_length - 1:
                print("End of line! Add character to scratchcard number and check for winners!")
                scratchcard_number = scratchcard_number + char
                if scratchcard_number not in winning_number_list:
                    print(f"Last number not a winner!")
                    scratchcard_number = ""
                    summed_card_scores.append(card_score)
                    print(summed_card_scores)
                    character_count = 0
                else:
                    print("Last number a winner! Checking win count")
                    if win_count == 0:
                        print("Final win count zero!")
                        card_score = 1
                        win_count = 1
                        summed_card_scores.append(card_score)
                        print(summed_card_scores)
                        character_count = 0
                    else:
                        print("Win count greater than zero!")
                        card_score = card_score * 2
                        win_count = win_count + 1
                        print(f"New card score: {card_score}")
                        print(f"New win count: {win_count}")
                        scratchcard_number = ""
                        summed_card_scores.append(card_score)
                        print(summed_card_scores)
                        character_count = 0

            elif re.match("\s",char):
                print("Whitespace character! Check scratchcard number!")
                if scratchcard_number == "":
                    character_count = character_count + 1
                    continue
                else:
                    if scratchcard_number not in winning_number_list:
                        print(f"Number not a winner!")
                        scratchcard_number = ""
                        character_count = character_count + 1
                    else:
                        print("Number a winner! Checking win count")
                        if win_count == 0:
                            print("Win count zero!")
                            card_score = 1
                            win_count = 1
                            character_count = character_count + 1
                            print(f"New card score: {card_score}")
                            print(f"New win count: {win_count}")
                            scratchcard_number = ""
                        else:
                            print("Win count greater than zero!")
                            card_score = card_score * 2
                            win_count = win_count + 1
                            character_count = character_count + 1
                            print(f"New card score: {card_score}")
                            print(f"New win count: {win_count}")
                            scratchcard_number = ""        
            else:
                scratchcard_number = scratchcard_number + char
                character_count = character_count + 1
            print(f"Current scratchcard number: {scratchcard_number}")
            print(f"Summed card scores: {summed_card_scores}")
    total_cards_score = sum(summed_card_scores)
    print(f"Total card scores: {total_cards_score}")

# Each row:
    # Store the card number
    # Split pre | data and add it to a winning number list
    # Split post | data and add it to a scratchcard numbers list
    # For each number in the scratchcard number list:
        # If it is not present in the winning number list, move on
        # If it is present in the winning number list:
            # Check the win count variable:
                # If 0: Change card score variable to 1 and win count + 1
                # If >0: card score * 2, win count + 1

def main():
    input_data = read_file(FILE_PATH)
    print(input_data)
    points_calculator = point_calculator(input_data)

if __name__ == "__main__":
    main()