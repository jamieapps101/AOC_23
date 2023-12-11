#! /usr/bin/env python3

from typing import List
import os

def nums_to_nums(nums: str) -> List[int]:
    return [int(n) for n in nums.split(" ") if len(n) > 0]


def line_to_score(line: str) -> int:
    line = line[line.find(":") + 1 :].strip()
    split = line.split("|")
    winning_nums = nums_to_nums(split[0])
    card_nums = nums_to_nums(split[1])
    return sum([n in winning_nums for n in card_nums])


def play_game(win_table: List[int]) -> List[int]:
    plays = [1 for _ in win_table]
    # for each scorecard to be played
    for i in range(len(plays)):
        # get the score it produces
        play_score = win_table[i]
        # increment the play counter for the relevent subsequent cards
        for j in range(i + 1, i + 1 + play_score):
            plays[j] += plays[i]
    return plays


def main():
    # build lookup of card number to win count
    wins = []
    path = os.environ.get("FILE_PATH","days/day_04/liv/input_data.txt")
    with open(path) as fp:
        for line in fp:
            line = line.strip()
            if line == "":
                break
            wins.append(line_to_score(line))
    plays = play_game(wins)
    print(sum(plays))


if __name__ == "__main__":
    main()


def test_line_to_score():
    data = [
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
        "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
        "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
        "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
        "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
        "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
    ]
    scores = [line_to_score(l) for l in data]
    assert scores == [4, 2, 2, 1, 0, 0]


# 14427616
