use std::num::ParseIntError;

#[derive(Debug, PartialEq, Clone)]
struct ScratchCard {
    index: u8,
    winning_numbers: Vec<u8>,
    card_numbers: Vec<u8>,
}

impl ScratchCard {
    fn eval(&self) -> u32 {
        self.winning_numbers
            .iter()
            .filter(|n| self.card_numbers.contains(n))
            .count() as u32
    }
}

#[derive(PartialEq, Debug)]
enum ScratchCardParseErr {
    ParseIntError(ParseIntError),
    UnexpectedCharacter(char),
}

impl From<ParseIntError> for ScratchCardParseErr {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum ScratchCardParseState {
    CardIndex,
    WinningNumbers,
    CardNumbers,
}

impl TryFrom<&str> for ScratchCard {
    type Error = ScratchCardParseErr;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut state = ScratchCardParseState::CardIndex;
        let mut buff = String::with_capacity(3);
        let mut source = value.chars();
        let mut index = 0u8;
        let mut winning_numbers = Vec::new();
        let mut card_numbers = Vec::new();
        loop {
            if let Some(c) = source.next() {
                match (state, c) {
                    (ScratchCardParseState::CardIndex, '0'..='9') => {
                        buff.push(c);
                    }
                    (ScratchCardParseState::CardIndex, ':') => {
                        index = buff.parse::<u8>()?;
                        buff.clear();
                        state = ScratchCardParseState::WinningNumbers;
                    }
                    (ScratchCardParseState::CardIndex, _) => {}
                    (ScratchCardParseState::WinningNumbers, ' ') => {
                        if !buff.is_empty() {
                            let n = buff.parse::<u8>()?;
                            winning_numbers.push(n);
                            buff.clear();
                        }
                    }
                    (ScratchCardParseState::CardNumbers, ' ') => {
                        if !buff.is_empty() {
                            let n = buff.parse::<u8>()?;
                            card_numbers.push(n);
                            buff.clear();
                        }
                    }
                    (
                        ScratchCardParseState::WinningNumbers | ScratchCardParseState::CardNumbers,
                        '0'..='9',
                    ) => {
                        buff.push(c);
                    }
                    (ScratchCardParseState::WinningNumbers, '|') => {
                        state = ScratchCardParseState::CardNumbers;
                    }
                    (ScratchCardParseState::WinningNumbers, _) => {
                        return Err(ScratchCardParseErr::UnexpectedCharacter(c))
                    }

                    _ => panic!("Undefined state"),
                }
            } else {
                if !buff.is_empty() {
                    let n = buff.parse::<u8>()?;
                    card_numbers.push(n);
                    buff.clear();
                }
                assert!(index != 0, "{:?}", index);

                return Ok(Self {
                    index,
                    winning_numbers,
                    card_numbers,
                });
            }
        }
    }
}

fn play_scratchcards(table: Vec<ScratchCard>) -> Vec<ScratchCard> {
    let mut execution_list = Vec::new();
    execution_list.extend_from_slice(table.as_slice());
    let mut execution_index = 0;
    loop {
        if let Some(scratchcard) = execution_list.get(execution_index) {
            execution_index += 1;
            let score = scratchcard.eval() as usize;
            // print!("card: {}  ", scratchcard.index);
            // println!("score: {}", score);
            let index = scratchcard.index as usize;
            execution_list.extend_from_slice(&table[index..(index + score)]);
        } else {
            break;
        }
        // if execution_index == 100 {
        // break;
        // }
    }
    execution_list
}

fn main() {
    let table = std::io::stdin()
        .lines()
        .filter_map(Result::ok)
        .filter(|s| !s.is_empty())
        .map(|s| ScratchCard::try_from(s.as_str()).unwrap())
        .collect::<Vec<ScratchCard>>();

    let total_list = play_scratchcards(table);
    println!("total: {}", total_list.len());
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse_scratchcard() {
        let s = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let ref_scratchcard = ScratchCard {
            index: 1,
            winning_numbers: vec![41, 48, 83, 86, 17],
            card_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
        };

        assert_eq!(Ok(ref_scratchcard), s.try_into());
    }

    #[test]
    fn test_example_data() {
        let example_data = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ];
        let s: Vec<u32> = example_data
            .into_iter()
            .map(|s| {
                let s = ScratchCard::try_from(s).unwrap();
                assert_eq!(s.winning_numbers.len(), 5);
                assert_eq!(s.card_numbers.len(), 8);
                dbg!(&s);
                s
            })
            .map(|s| s.eval())
            .collect::<Vec<u32>>();
        assert_eq!(s, vec![4, 2, 2, 1, 0, 0]);
    }
    #[test]
    fn test_game_play() {
        let data = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ];
        let table = data
            .into_iter()
            .map(|s| ScratchCard::try_from(s).unwrap())
            .collect();
        let total = play_scratchcards(table);
        assert_eq!(total.len(), 30);
        let card_indexes = total.into_iter().map(|s| s.index).collect::<Vec<u8>>();
        dbg!(&card_indexes);
        assert_eq!(card_indexes.iter().filter(|&s| s == &1).count(), 1);
        assert_eq!(card_indexes.iter().filter(|&s| s == &2).count(), 2);
        assert_eq!(card_indexes.iter().filter(|&s| s == &3).count(), 4);
        assert_eq!(card_indexes.iter().filter(|&s| s == &4).count(), 8);
        assert_eq!(card_indexes.iter().filter(|&s| s == &5).count(), 14);
        assert_eq!(card_indexes.iter().filter(|&s| s == &6).count(), 1);
    }
}
