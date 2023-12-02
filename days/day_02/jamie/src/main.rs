#[derive(Debug, PartialEq)]
struct Game {
    index: u32,
    rounds: Vec<Round>,
}

#[derive(Debug, PartialEq)]
struct Round {
    red: u32,
    blue: u32,
    green: u32,
}

trait LoadGames<S: Iterator<Item = String>> {
    fn load_games(self) -> GameLoader<S>;
}

struct GameLoader<S: Iterator<Item = String>> {
    source: S,
}

impl<S: Iterator<Item = String>> LoadGames<S> for S {
    fn load_games(self) -> GameLoader<S> {
        GameLoader { source: self }
    }
}

enum ParserState {
    GameIndex,
    Number,
    Colour,
}

#[derive(Debug, PartialEq)]
enum ParserError {
    ExpectingNum(String),
    ExpectingColour(String),
    UnexpectedCharacter(char),
}

impl<S: Iterator<Item = String>> Iterator for GameLoader<S> {
    type Item = Result<Game, ParserError>;
    fn next(&mut self) -> Option<Self::Item> {
        let s = self.source.next()?;
        let s_chars = s.chars().skip(5).filter(|c| c != &' ').chain(Some(';')); // skip "Game "
        let mut parser_state = ParserState::GameIndex;
        let mut buffer = String::with_capacity(10);
        let mut index = 0;
        let mut rounds = Vec::new();
        let mut colour_count_store = 0;
        let mut red = None;
        let mut blue = None;
        let mut green = None;
        for c in s_chars {
            match parser_state {
                ParserState::GameIndex => {
                    if c.is_ascii_digit() {
                        buffer.push(c);
                    } else {
                        if c != ':' {
                            return Some(Err(ParserError::UnexpectedCharacter(c)));
                        }
                        match buffer.parse::<u32>() {
                            Ok(i) => index = i,
                            Err(_r) => return Some(Err(ParserError::ExpectingNum(buffer))),
                        }
                        buffer.clear();
                        parser_state = ParserState::Number;
                    }
                }
                ParserState::Number => {
                    if c.is_ascii_digit() {
                        buffer.push(c);
                    } else {
                        match buffer.parse::<u32>() {
                            Ok(i) => colour_count_store = i,
                            Err(_r) => return Some(Err(ParserError::ExpectingNum(buffer))),
                        }
                        buffer.clear();
                        buffer.push(c);
                        parser_state = ParserState::Colour;
                    }
                }
                ParserState::Colour => {
                    if c.is_alphabetic() {
                        buffer.push(c);
                    } else {
                        match buffer.as_ref() {
                            "red" => red = Some(colour_count_store),
                            "blue" => blue = Some(colour_count_store),
                            "green" => green = Some(colour_count_store),
                            _ => return Some(Err(ParserError::ExpectingColour(buffer))),
                        }
                        buffer.clear();
                        if c == ';' {
                            // we've hit the end of a round
                            let round = Round {
                                red: std::mem::take(&mut red).unwrap_or_default(),
                                green: std::mem::take(&mut green).unwrap_or_default(),
                                blue: std::mem::take(&mut blue).unwrap_or_default(),
                            };
                            rounds.push(round);
                        }
                        parser_state = ParserState::Number;
                    }
                }
            }
        }
        debug_assert_eq!(buffer, String::new());
        if red.is_some() || blue.is_some() || green.is_some() {
            rounds.push(Round {
                red: red.unwrap_or_default(),
                green: green.unwrap_or_default(),
                blue: blue.unwrap_or_default(),
            })
        }
        Some(Ok(Game { index, rounds }))
    }
}

fn main() {
    let stdin = std::io::stdin()
        .lines()
        .map_while(Result::ok)
        .filter(|s| !s.is_empty());
    let sum_of_ids = stdin
        .load_games()
        .map(|r| r.unwrap())
        .map(|g| {
            let mut min_round = Round {
                red: u32::MIN,
                green: u32::MIN,
                blue: u32::MIN,
            };
            g.rounds.iter().for_each(|r| {
                min_round.red = min_round.red.max(r.red);
                min_round.blue = min_round.blue.max(r.blue);
                min_round.green = min_round.green.max(r.green);
            });
            let power = min_round.red * min_round.blue * min_round.green;
            power
        })
        .sum::<u32>();
    println!("sum_of_ids: {sum_of_ids}");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_game_loader() {
        let test_data = vec![
            "Game 1: 7 blue, 6 green, 3 red; 3 red, 5 green, 1 blue; 1 red, 5 green, 8 blue; 3 red, 1 green, 5 blue".to_owned(),
            "Game 2: 9 green, 1 blue, 12 red; 1 blue, 18 green, 8 red; 2 blue, 6 green, 13 red; 3 blue, 13 red, 7 green; 5 blue, 4 red, 4 green; 6 blue, 7 green, 4 red".to_owned(),
            "Game 3: 5 blue, 9 red, 14 green; 10 green, 3 blue; 11 red, 2 blue, 8 green; 5 red, 2 blue; 5 blue, 7 green, 8 red".to_owned(),
            "Game 4: 2 red, 3 blue, 2 green; 17 green, 6 blue, 1 red; 3 blue, 5 green, 1 red; 4 red, 1 blue, 16 green; 5 red, 4 blue, 13 green; 14 green, 5 blue, 6 red".to_owned(),
            "Game 5: 3 red, 17 green, 10 blue; 9 blue, 5 green; 14 green, 9 blue, 11 red".to_owned(),
        ];
        let games: Vec<Game> = test_data
            .into_iter()
            .load_games()
            .map(|r| r.unwrap())
            .collect();
        assert_eq!(games.len(), 5);
        assert_eq!(
            games.iter().map(|g| g.rounds.len()).collect::<Vec<usize>>(),
            vec![4, 6, 5, 6, 3]
        );
        assert_eq!(
            games[0].rounds[0],
            Round {
                blue: 7,
                green: 6,
                red: 3
            }
        );
        assert_eq!(
            games[4],
            Game {
                index: 5,
                rounds: vec![
                    Round {
                        red: 3,
                        green: 17,
                        blue: 10
                    },
                    Round {
                        red: 0,
                        green: 5,
                        blue: 9
                    },
                    Round {
                        red: 11,
                        green: 14,
                        blue: 9
                    },
                ]
            }
        )
    }
}
