from typing import List
import re
import os

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

def cards_calculator(input_data: List[str]) -> int:
    card_number = 0
    character_count = 0
    all_card_numbers = []
    card_copies_reference = []
    for row in input_data:
        card_number = int(re.search("[0-9]{1,100}",row).group())
        all_card_numbers.append(card_number)
    # print(f"All card numbers: {all_card_numbers}")
    # get the last card number
    max_card_number = all_card_numbers[len(all_card_numbers)-1]
    # print(f"Max card number = {max_card_number}")
    ###### C
    for row in input_data:
    ###### /C
        ###### Z
        card_number = int(re.search("[0-9]{1,100}",row).group())
        # print(f"Card number is {card_number}")
        card_copies_reference.append(card_number)
        # print(f"Start of row card_copies_reference: {card_copies_reference}")
        row = re.sub("  "," 0",row)
        # print(row)
        row = re.sub("Card [0-9]{1,1000}: ","",row)
        # print(row)
        split_string = row.split("|")
        # print(f"Split string is {split_string}")
        winning_number_list = split_string[0].lstrip()
        scratchcard_number_list = split_string[1].rstrip()
        # print(f"Winning numbers are: {winning_number_list}")
        # print(f"Scratchcard numbers are: {scratchcard_number_list}")
        scratchcard_number = ""
        win_count = 0
        spawned_card_numbers = []
        for char in scratchcard_number_list:
            line_length = len(scratchcard_number_list)
            # print(f"Line length: {line_length}")
            # print(f"Current char: {char}")
            # print(f"Character count: {character_count}")
            if character_count == line_length - 1:
                # print("End of line! Add character to scratchcard number and check for winners!")
                scratchcard_number = scratchcard_number + char
                ###### /Z
                ####### A
                if scratchcard_number not in winning_number_list:
                    # print(f"Last number not a winner!")
                    scratchcard_number = ""
                    # print(f"Win count for card: {win_count}")
                    # Check how many instances of the card there are:
                    instances_of_card = card_copies_reference.count(card_number)
                    # print(f"Number of card instances: {instances_of_card}") 
                    # Get spawned card numbers somehow??
                    # for number_of_cards in range(0,instances_of_card):
                    ######### /A
                    ######### B
                    if win_count == 1:
                        next_card = card_number + 1
                        spawned_card_numbers.append(next_card)
                        # print(f"Win count 1, spawned numbers: {spawned_card_numbers}")
                    else:
                        for number in range(card_number + 1, card_number + win_count + 1):
                            spawned_card_numbers.append(number)
                        # print(f"Win count > 1, spawned card numbers: {spawned_card_numbers}")
                    for number in range(instances_of_card):
                        card_copies_reference.extend(spawned_card_numbers)
                            # card_copies_reference.extend(spawned_card_numbers)
                    # print(card_copies_reference)
                    ######### /B
                    character_count = 0
                else:
                    # print("Last number a winner! Checking win count")
                    ####### A
                    if win_count == 0:
                        # print("Final win count zero!")
                        card_score = 1
                        win_count = 1
                        # print(f"Win count for card: {win_count}")
                        # Get spawned card numbers somehow??
                        instances_of_card = card_copies_reference.count(card_number)
                        # print(f"Number of card instances: {instances_of_card}") 
                        # Get spawned card numbers somehow??
                        if win_count == 1:
                            next_card = card_number + 1
                            if next_card > max_card_number:
                                # continue
                                pass
                            else:
                                ####### B
                                spawned_card_numbers.append(next_card)
                                # print(f"Win count 1, spawned numbers: {spawned_card_numbers}")
                                # card_copies_reference.extend(spawned_card_numbers)
                                ###### /B
                        else:
                            ###### B
                            for number in range(card_number + 1, card_number + win_count + 1):
                                if number <= max_card_number:
                                    spawned_card_numbers.append(number)
                                else:
                                    break
                            ###### /B
                            # print(f"Win count > 1, spawned card numbers: {spawned_card_numbers}")
                        ###### B
                        for number in range(instances_of_card):
                            card_copies_reference.extend(spawned_card_numbers)
                        ###### /B
                                # card_copies_reference.extend(spawned_card_numbers)
                        # print(card_copies_reference)
                        ###### Z
                        character_count = 0
                        ###### /Z
                    else:
                        # print("Win count greater than zero!")
                        card_score = card_score * 2
                        win_count = win_count + 1
                        # print(f"New card score: {card_score}")
                        # print(f"New win count: {win_count}")
                        scratchcard_number = ""
                        # print(f"Win count for card: {win_count}")
                        # Get spawned card numbers somehow??
                        instances_of_card = card_copies_reference.count(card_number)
                        # print(f"Number of card instances: {instances_of_card}") 
                        # Get spawned card numbers somehow??
                        ###### B
                        if win_count == 1:
                            next_card = card_number + 1
                            if next_card > max_card_number:
                                # continue
                                pass
                            else:
                                spawned_card_numbers.append(next_card)
                                # print(f"Win count 1, spawned numbers: {spawned_card_numbers}")
                                # TODO: This might be missing the content of line 97
                            # print(card_copies_reference)
                        else:
                            for number in range(card_number + 1, card_number + win_count + 1):
                                if number <= max_card_number:
                                    spawned_card_numbers.append(number)
                                else:
                                    break
                            # print(f"Win count > 1, spawned card numbers: {spawned_card_numbers}")
                        for number in range(instances_of_card):
                            card_copies_reference.extend(spawned_card_numbers)
                        ###### /B
                        ###### Z
                        # print(card_copies_reference)
                        character_count = 0

            elif re.match("\s",char):
                # print("Whitespace character! Check scratchcard number!")
                if scratchcard_number == "":
                    character_count = character_count + 1
                else:
                    ##### A
                    if scratchcard_number not in winning_number_list:
                        # print(f"Number not a winner!")
                        scratchcard_number = ""
                        character_count = character_count + 1
                    else:
                        # print("Number a winner! Checking win count")
                        ###### A
                        if win_count == 0:
                            # print("Win count zero!")
                            card_score = 1
                            win_count = 1
                            character_count = character_count + 1
                            # print(f"New card score: {card_score}")
                            # print(f"New win count: {win_count}")
                            scratchcard_number = ""
                        else:
                            # print("Win count greater than zero!")
                            card_score = card_score * 2
                            win_count = win_count + 1
                            character_count = character_count + 1
                            # print(f"New card score: {card_score}")
                            # print(f"New win count: {win_count}")
                            scratchcard_number = ""
                        ##### /A        
            else:
                scratchcard_number = scratchcard_number + char
                character_count = character_count + 1
            # print(f"Current scratchcard number: {scratchcard_number}")
    #         print(f"Summed card scores: {summed_card_scores}")
    # print(f"Card copies reference: {card_copies_reference}")
    total_cards = len(card_copies_reference)
    print(f"Total number of cards: {total_cards}")

# Win_score dictates how many new cards spawn.
# Already have the win scores for each card
# Calculate win scores for original cards
# Card count starts at the maximum number of cards
# For each card: add spawned card numbers to a list at end. At start, check list for how many copies of the current card there are.

# Implement highest card stop: if numbers generated are greater than max, discard (done)
# Implement list check to determine how many cards to check


# Card 1 has four matching numbers, so you win one copy each of the next four cards: cards 2, 3, 4, and 5.
# Your original card 2 has two matching numbers, so you win one copy each of cards 3 and 4.
# Your copy of card 2 also wins one copy each of cards 3 and 4.
# Your four instances of card 3 (one original and three copies) have two matching numbers, so you win four copies each of cards 4 and 5.
# Your eight instances of card 4 (one original and seven copies) have one matching number, so you win eight copies of card 5.
# Your fourteen instances of card 5 (one original and thirteen copies) have no matching numbers and win no more cards.
# Your one instance of card 6 (one original) has no matching numbers and wins no more cards.


def main():
    input_data = read_file(FILE_PATH)
    # print(input_data)
    points_calculator = cards_calculator(input_data)

if __name__ == "__main__":
    main()