use std::iter::Enumerate;

trait ReadSymbols<S: Iterator<Item = char>> {
    fn read_symbols(self, line_index: usize) -> SymbolReader<S>;
}

struct SymbolReader<S: Iterator> {
    source: Peekable<Enumerate<S>>,
    line_index: usize,
}

impl<S: Iterator<Item = char>> ReadSymbols<S> for S {
    fn read_symbols(self, line_index: usize) -> SymbolReader<S> {
        SymbolReader {
            source: self.enumerate().peekable(),
            line_index,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Token {
    token_type: TokenType,
    token_len: usize,
    char_index: usize,
    line_index: usize,
}

impl Token {
    fn is_number(&self) -> bool {
        matches!(self.token_type, TokenType::Number(_))
    }

    fn get_number(&self) -> Option<u32> {
        if let TokenType::Number(n) = self.token_type {
            Some(n)
        } else {
            None
        }
    }

    fn is_symbol(&self) -> bool {
        matches!(self.token_type, TokenType::Symbol(_))
    }

    // fn get_symbol(&self) -> Option<char> {
    //     if let TokenType::Symbol(c) = self.token_type {
    //         Some(c)
    //     } else {
    //         None
    //     }
    // }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
enum TokenType {
    Number(u32),
    Symbol(char),
}

impl From<(TokenType, usize, usize, usize)> for Token {
    fn from(value: (TokenType, usize, usize, usize)) -> Self {
        Self {
            token_type: value.0,
            token_len: value.1,
            char_index: value.2,
            line_index: value.3,
        }
    }
}
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
enum ReaderState {
    Unknown,
    Number,
}

use std::iter::Peekable;

impl<S: Iterator<Item = char>> Iterator for SymbolReader<S> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = String::with_capacity(5);
        let mut state = ReaderState::Unknown;
        let mut index_state = 0;
        while let Some((index_ref, c_ref)) = self.source.peek() {
            // // dbg!((&state, index_ref, c_ref));
            match (state, c_ref) {
                // do nothing
                (ReaderState::Unknown, '.') => {
                    let _ = self.source.next().unwrap();
                }
                (ReaderState::Unknown, '0'..='9') => {
                    state = ReaderState::Number;
                    let (i, c) = self.source.next().unwrap();
                    index_state = i;
                    buffer.push(c);
                }
                (ReaderState::Unknown, _) => {
                    // state = ReaderState::Symbol;
                    // we only expect symbols to be a single char
                    // buffer.push(self.source.next().unwrap().1);
                    let (index, c) = self.source.next().unwrap();
                    let token = Token {
                        token_type: TokenType::Symbol(c),
                        token_len: 1,
                        char_index: index,
                        line_index: self.line_index,
                    };
                    return Some(token);
                }
                (ReaderState::Number, '0'..='9') => {
                    let (i, c) = self.source.next().unwrap();
                    // // dbg!((i, c));
                    index_state = i;
                    buffer.push(c);
                }
                (ReaderState::Number, _) => {
                    // change from a numerical value to something else,
                    // must be end of number so return a number tokwn
                    let token = Token {
                        token_type: TokenType::Number(buffer.parse::<u32>().unwrap()),
                        token_len: buffer.len(),
                        char_index: index_ref - buffer.len(),
                        line_index: self.line_index,
                    };
                    return Some(token);
                }
            }
        }
        if !buffer.is_empty() && state == ReaderState::Number {
            // // dbg!(index_state);
            // // dbg!(buffer.len());
            let token = Token {
                token_type: TokenType::Number(buffer.parse::<u32>().unwrap()),
                token_len: buffer.len(),
                char_index: index_state + 1 - buffer.len(),
                line_index: self.line_index,
            };
            return Some(token);
        }
        None
    }
}

trait FindPartNumbers<S: Iterator<Item = Vec<Token>>> {
    fn find_part_numbers(self) -> PartNumberFinder<S>;
}

struct PartNumberFinder<S: Iterator<Item = Vec<Token>>> {
    source: S,
    n_3_line: Option<Vec<Token>>,
    n_2_line: Option<Vec<Token>>,
    n_1_line: Option<Vec<Token>>,
    buffer: Vec<Option<Token>>,
    buffer_head: usize,
}

impl<S: Iterator<Item = Vec<Token>>> FindPartNumbers<S> for S {
    fn find_part_numbers(self) -> PartNumberFinder<S> {
        PartNumberFinder {
            source: self,
            n_3_line: None,
            n_2_line: None,
            n_1_line: None,
            buffer: Vec::with_capacity(10),
            buffer_head: 0,
        }
    }
}

impl<S: Iterator<Item = Vec<Token>>> PartNumberFinder<S> {
    fn get_next_from_buffer(&mut self) -> Option<Token> {
        if self.buffer_head < self.buffer.len() {
            let rtn_val = std::mem::take(&mut self.buffer[self.buffer_head]);
            self.buffer_head += 1;
            rtn_val
        } else {
            None
        }
    }

    fn reset_buffer(&mut self) {
        self.buffer.clear();
        self.buffer_head = 0;
    }
}

impl Token {
    fn is_adjacent(&self, other: &Self) -> bool {
        // three cases
        //o       ***     ***     ***
        //s  *****       *****       *****

        // // dbg!(self);
        // // dbg!(other);
        let right_side = (self.char_index + self.token_len) >= other.char_index;
        // // dbg!(right_side);
        let left_side = self.char_index <= (other.char_index + other.token_len);
        // // dbg!(left_side);
        right_side && left_side
    }
}

fn adjacent_numbers(
    number_line: &[Token],
    previous_line: Option<&[Token]>,
    next_line: Option<&[Token]>,
) -> Vec<Token> {
    let mut out = Vec::new();
    // scan previous line
    for number in number_line.iter().filter(|t| t.is_number()) {
        // // dbg!(&number);
        let mut skip = false;
        if let Some(previous_line) = previous_line {
            for symbol in previous_line.iter().filter(|t| t.is_symbol()) {
                // // dbg!(&symbol);
                if number.is_adjacent(symbol) {
                    out.push(number.clone());
                    skip = true;
                    break;
                }
            }
        }
        if !skip {
            for symbol in number_line.iter().filter(|t| t.is_symbol()) {
                // // dbg!(&symbol);
                if number.is_adjacent(symbol) {
                    out.push(number.clone());
                    skip = true;
                    break;
                }
            }
        }
        if !skip {
            if let Some(next_line) = next_line {
                for symbol in next_line.iter().filter(|t| t.is_symbol()) {
                    // // dbg!(&symbol);
                    if number.is_adjacent(symbol) {
                        out.push(number.clone());
                        break;
                    }
                }
            }
        }
    }

    out
}

impl<S: Iterator<Item = Vec<Token>>> Iterator for PartNumberFinder<S> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // try to get a token from the buffer
            if let Some(t) = self.get_next_from_buffer() {
                return Some(t);
            }
            // if no token in the buffer, reset the buffer and loop until there is something
            self.reset_buffer();

            match (
                self.n_1_line.as_mut(),
                self.n_2_line.as_mut(),
                self.n_3_line.as_mut(),
            ) {
                (None, None, None) => {
                    self.n_1_line = self.source.next()?.into();
                    let mut last_token_opt: Option<Token> = None;
                    for token in self.n_1_line.as_ref().unwrap().iter() {
                        if let Some(last_token) = last_token_opt.as_mut() {
                            // if one token is a symbol and the other is a number and they're adjacent
                            if (token.is_number() ^ last_token.is_number())
                                && token.is_adjacent(last_token)
                            {
                                self.buffer.push(token.clone().into());
                            }
                        } else {
                            last_token_opt = Some(token.clone());
                        }
                    }
                }
                (Some(l), None, None) => {
                    let n_2_line = self.source.next()?;
                    // for each number on the middle line, look for an adjacent symbol on the
                    // previous line and then the current line
                    let numbers_with_adjacency =
                        adjacent_numbers(l.as_slice(), None, Some(n_2_line.as_slice()));
                    // dbg!(&numbers_with_adjacency);
                    self.buffer
                        .extend(numbers_with_adjacency.into_iter().map(|r| Some(r)));
                    self.n_2_line = n_2_line.into();
                }
                (Some(_), Some(_), None) => self.n_3_line = self.source.next()?.into(),

                (Some(l1), Some(l2), Some(l3)) => {
                    // println!("=================");
                    // for (i, l) in [l1.clone(), l2.clone(), l3.clone()].into_iter().enumerate() {
                    //     let line = l
                    //         .iter()
                    //         .filter_map(|t| t.get_number())
                    //         .map(|n| format!(" {n} "))
                    //         .collect::<String>();
                    //     println!("l{i}: {line}");
                    // }
                    // println!("=================");
                    // // dbg!("Made it here");
                    // for each number on the middle line, look for an adjacent symbol on the
                    // previous line and then the current line and the next line
                    let numbers =
                        adjacent_numbers(l2.as_slice(), Some(l1.as_slice()), Some(l3.as_slice()));
                    self.buffer.extend(numbers.into_iter().map(|r| Some(r)));
                    // // dbg!("and here");
                    if let Some(next_line) = self.source.next() {
                        // dbg!("a next line to process");
                        let _ = std::mem::replace(
                            &mut self.n_1_line,
                            std::mem::replace(
                                &mut self.n_2_line,
                                std::mem::replace(&mut self.n_3_line, Some(next_line)),
                            ),
                        );
                    } else {
                        // dbg!("no next line");
                        // take the first line in the buffer as an indicator to the
                        // statemachine/match statement to only look for numbers in the last
                        // line
                        // *l1 = None;
                        let _ = std::mem::take(&mut self.n_1_line);
                    }
                }
                (None, Some(l2), Some(l3)) => {
                    // we just need to scan the last line now
                    let numbers_with_adjacency =
                        adjacent_numbers(l3.as_slice(), Some(l2.as_slice()), None);
                    // dbg!(&numbers_with_adjacency);
                    self.buffer
                        .extend(numbers_with_adjacency.into_iter().map(|r| Some(r)));
                    self.n_2_line = None;
                }
                _ => return None,
            }
        }
    }
}

fn main() {
    // let numbers =  Vec::<Token>::new();
    // let symbols =  Vec::<Token>::new();
    let part_numbers: u32 = std::io::stdin()
        .lines()
        .map_while(Result::ok)
        .enumerate()
        .map(|(i, line)| line.chars().read_symbols(i).collect())
        .find_part_numbers()
        .filter_map(|t| t.get_number())
        .sum();
    println!("part_numbers: {part_numbers}");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_tokeniser() {
        let s = "..234..+..3-.++2";
        let tokens = s.chars().read_symbols(0).collect::<Vec<Token>>();
        let tokens_ref = vec![
            (TokenType::Number(234), 3, 2, 0).into(),
            (TokenType::Symbol('+'), 1, 7, 0).into(),
            (TokenType::Number(3), 1, 10, 0).into(),
            (TokenType::Symbol('-'), 1, 11, 0).into(),
            (TokenType::Symbol('+'), 1, 13, 0).into(),
            (TokenType::Symbol('+'), 1, 14, 0).into(),
            (TokenType::Number(2), 1, 15, 0).into(),
        ];
        for (t, t_r) in tokens.iter().zip(tokens_ref.iter()) {
            assert_eq!(t, t_r)
        }
        assert_eq!(tokens.len(), tokens_ref.len());
    }

    #[test]
    fn test_is_adjacent() {
        // token source
        let token_text = vec![
            "111.......111.",
            ".....111......",
            "..............",
            "11.11.11.11.11",
            "..11..11..11..",
            "..............",
            ".........111..",
            "........11111.",
            "..114...114...",
            "*...........*.",
        ];
        let tokens: Vec<Vec<Token>> = token_text
            .into_iter()
            .enumerate()
            .map(|(i, s)| s.chars().read_symbols(i).collect())
            .collect();
        assert_eq!(tokens[0][0].is_adjacent(&tokens[0][1]), false);
        assert_eq!(tokens[0][0].is_adjacent(&tokens[1][0]), false);
        assert_eq!(tokens[1][0].is_adjacent(&tokens[0][1]), false);
        assert_eq!(tokens[4][0].is_adjacent(&tokens[3][0]), true);
        assert_eq!(tokens[4][0].is_adjacent(&tokens[3][1]), true);
        assert_eq!(tokens[4][1].is_adjacent(&tokens[3][2]), true);
        assert_eq!(tokens[4][2].is_adjacent(&tokens[3][3]), true);
        assert_eq!(tokens[4][2].is_adjacent(&tokens[3][4]), true);
        assert_eq!(tokens[7][0].is_adjacent(&tokens[3][4]), true);
        // dbg!(&tokens[9][0]);
        // dbg!(&tokens[8][0]);
        assert_eq!(tokens[9][0].is_adjacent(&tokens[8][0]), false);
        // dbg!(&tokens[9][1]);
        // dbg!(&tokens[8][1]);
        assert_eq!(tokens[9][1].is_adjacent(&tokens[8][1]), false);
    }

    #[test]
    fn test_adjacency_finder() {
        let input = vec![
            "467..114..".to_owned(),
            "...*......".to_owned(),
            "..35..633.".to_owned(),
            "......#...".to_owned(),
            "617*......".to_owned(),
            ".....+.58.".to_owned(),
            "..592.....".to_owned(),
            "......755.".to_owned(),
            "...$.*....".to_owned(),
            ".664.598..".to_owned(),
        ];
        let part_numbers: Vec<Token> = input
            .into_iter()
            .enumerate()
            .map(|(i, line)| line.chars().read_symbols(i).collect())
            .find_part_numbers()
            .collect();
        println!("part_numbers: {part_numbers:?}");
        let values = part_numbers
            .iter()
            .filter_map(|t| {
                if let TokenType::Number(n) = t.token_type {
                    Some(n)
                } else {
                    None
                }
            })
            .collect::<Vec<u32>>();
        let sum: u32 = values.iter().sum();
        assert_eq!(values, vec![467, 35, 633, 617, 592, 755, 664, 598]);
        assert_eq!(sum, 4361);
    }
}
