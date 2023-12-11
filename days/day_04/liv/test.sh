#! /usr/bin/env bash

# LINES=191

export FILE_PATH=days/day_04/liv/line_16.txt

# cat days/day_04/liv/input_data.txt | tail -n "${LINES}" > ${FILE_PATH}

LIV_RES=`time python3 ./days/day_04/liv/main.py | sed -n -r 's/^Total number of cards: ([0-9]+)$/\1/p'`
echo "LIV_RES: ${LIV_RES}"
JAMIE_RES=`time ./days/day_04/jamie/quick.py`
echo "JAMIE_RES: ${JAMIE_RES}"
