use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
struct ScratchCard {
    winning_numbers: Vec<u8>,
    card_numbers: Vec<u8>,
}

impl ScratchCard {
    fn eval(self) -> u32 {
        let matches = self
            .winning_numbers
            .iter()
            .filter(|n| self.card_numbers.contains(n))
            .count() as u32;
        if matches == 0 {
            0
        } else {
            2u32.pow(matches - 1)
        }
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
    WinningNumbers,
    CardNumbers,
    Start,
}

impl TryFrom<&str> for ScratchCard {
    type Error = ScratchCardParseErr;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut state = ScratchCardParseState::Start;
        let mut buff = String::with_capacity(3);
        let mut source = value.chars();
        let mut winning_numbers = Vec::new();
        let mut card_numbers = Vec::new();
        loop {
            if let Some(c) = source.next() {
                match (state, c) {
                    (ScratchCardParseState::Start, ':') => {
                        state = ScratchCardParseState::WinningNumbers;
                    }
                    (ScratchCardParseState::Start, _) => {}
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

                return Ok(Self {
                    winning_numbers,
                    card_numbers,
                });
            }
        }
    }
}

fn main() {
    let score: u32 = std::io::stdin()
        .lines()
        .filter_map(Result::ok)
        .map(|s| ScratchCard::try_from(s.as_str()).unwrap())
        .map(|s| s.eval())
        .sum();
    println!("score: {score}");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse_scratchcard() {
        let s = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let ref_scratchcard = ScratchCard {
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
        assert_eq!(s, vec![8, 2, 2, 1, 0, 0]);
    }
}
