#[derive(Debug, PartialEq)]
pub struct Game {
    pub index: u32,
    pub rounds: Vec<Round>,
}

#[derive(Debug, PartialEq)]
pub struct Round {
    pub red: u32,
    pub blue: u32,
    pub green: u32,
}

pub trait LoadGames<I: AsRef<str>, S: Iterator<Item = I>> {
    fn load_games(self) -> GameLoader<I, S>;
}

impl<I: AsRef<str>, S: Iterator<Item = I>> LoadGames<I, S> for S {
    fn load_games(self) -> GameLoader<I, S> {
        self.into()
    }
}

enum ParserMode {
    GameIndex,
    Number,
    Colour,
}

#[derive(Debug, PartialEq)]
pub enum ParserError {
    ExpectingNum(String),
    ExpectingColour(String),
    UnexpectedCharacter(char),
}

pub struct GameLoader<I: AsRef<str>, S: Iterator<Item = I>> {
    source: S,
    state: ParserState,
}

struct ParserState {
    parser_mode: ParserMode,
    buffer: String,
    index: u32,
    rounds: Vec<Round>,
    colour_count_store: u32,
    red: Option<u32>,
    green: Option<u32>,
    blue: Option<u32>,
}

impl Default for ParserState {
    fn default() -> Self {
        Self {
            parser_mode: ParserMode::GameIndex,
            buffer: String::with_capacity(5),
            rounds: Vec::with_capacity(5),
            index: 0,
            colour_count_store: 0,
            red: None,
            green: None,
            blue: None,
        }
    }
}

impl<I: AsRef<str>, S: Iterator<Item = I>> From<S> for GameLoader<I, S> {
    fn from(source: S) -> Self {
        Self {
            source,
            state: ParserState::default(),
        }
    }
}

impl<I: AsRef<str>, S: Iterator<Item = I>> GameLoader<I, S> {
    pub fn next_game_fast(&mut self) -> Option<Result<Game, ParserError>> {
        let s = self.source.next()?;
        let s_chars = s
            .as_ref()
            .chars()
            .skip(5)
            .filter(|c| c != &' ')
            .chain(Some(';')); // skip "Game "
        for c in s_chars {
            match self.state.parser_mode {
                ParserMode::GameIndex => {
                    if c.is_ascii_digit() {
                        self.state.buffer.push(c);
                    } else {
                        if c != ':' {
                            return Some(Err(ParserError::UnexpectedCharacter(c)));
                        }
                        match self.state.buffer.parse::<u32>() {
                            Ok(i) => self.state.index = i,
                            Err(_r) => {
                                return Some(Err(ParserError::ExpectingNum(
                                    self.state.buffer.clone(),
                                )))
                            }
                        }
                        self.state.buffer.clear();
                        self.state.parser_mode = ParserMode::Number;
                    }
                }
                ParserMode::Number => {
                    if c.is_ascii_digit() {
                        self.state.buffer.push(c);
                    } else {
                        match self.state.buffer.parse::<u32>() {
                            Ok(i) => self.state.colour_count_store = i,
                            Err(_r) => {
                                return Some(Err(ParserError::ExpectingNum(
                                    self.state.buffer.clone(),
                                )))
                            }
                        }
                        self.state.buffer.clear();
                        self.state.buffer.push(c);
                        self.state.parser_mode = ParserMode::Colour;
                    }
                }
                ParserMode::Colour => {
                    if c.is_alphabetic() {
                        self.state.buffer.push(c);
                    } else {
                        match self.state.buffer.as_ref() {
                            "red" => self.state.red = Some(self.state.colour_count_store),
                            "blue" => self.state.blue = Some(self.state.colour_count_store),
                            "green" => self.state.green = Some(self.state.colour_count_store),
                            _ => {
                                return Some(Err(ParserError::ExpectingColour(
                                    self.state.buffer.clone(),
                                )))
                            }
                        }
                        self.state.buffer.clear();
                        if c == ';' {
                            // we've hit the end of a round
                            let round = Round {
                                red: std::mem::take(&mut self.state.red).unwrap_or_default(),
                                green: std::mem::take(&mut self.state.green).unwrap_or_default(),
                                blue: std::mem::take(&mut self.state.blue).unwrap_or_default(),
                            };
                            self.state.rounds.push(round);
                        }
                        self.state.parser_mode = ParserMode::Number;
                    }
                }
            }
        }
        debug_assert_eq!(self.state.buffer, String::new());
        if self.state.red.is_some() || self.state.blue.is_some() || self.state.green.is_some() {
            self.state.rounds.push(Round {
                red: self.state.red.unwrap_or_default(),
                green: self.state.green.unwrap_or_default(),
                blue: self.state.blue.unwrap_or_default(),
            })
        }
        Some(Ok(Game {
            index: self.state.index,
            rounds: std::mem::replace(&mut self.state.rounds, Vec::with_capacity(5)),
        }))
    }
    pub fn next_game(&mut self) -> Option<Result<Game, ParserError>> {
        let s = self.source.next()?;
        let s_chars = s
            .as_ref()
            .chars()
            .skip(5)
            .filter(|c| c != &' ')
            .chain(Some(';')); // skip "Game "
        let mut parser_mode = ParserMode::GameIndex;
        let mut buffer = String::with_capacity(10);
        let mut index = 0;
        let mut rounds = Vec::new();
        let mut colour_count_store = 0;
        let mut red = None;
        let mut blue = None;
        let mut green = None;
        for c in s_chars {
            match parser_mode {
                ParserMode::GameIndex => {
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
                        parser_mode = ParserMode::Number;
                    }
                }
                ParserMode::Number => {
                    if c.is_ascii_digit() {
                        buffer.push(c);
                    } else {
                        match buffer.parse::<u32>() {
                            Ok(i) => colour_count_store = i,
                            Err(_r) => return Some(Err(ParserError::ExpectingNum(buffer))),
                        }
                        buffer.clear();
                        buffer.push(c);
                        parser_mode = ParserMode::Colour;
                    }
                }
                ParserMode::Colour => {
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
                        parser_mode = ParserMode::Number;
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

impl<I: AsRef<str>, S: Iterator<Item = I>> Iterator for GameLoader<I, S> {
    type Item = Result<Game, ParserError>;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_game_fast()
    }
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
        let mut games: Vec<Game> = Vec::with_capacity(5);
        let mut loader: GameLoader<_, _> = test_data.iter().into();
        while let Some(g) = loader.next_game() {
            games.push(g.unwrap());
        }
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

    #[test]
    fn test_game_loader_fast() {
        let test_data = vec![
            "Game 1: 7 blue, 6 green, 3 red; 3 red, 5 green, 1 blue; 1 red, 5 green, 8 blue; 3 red, 1 green, 5 blue".to_owned(),
            "Game 2: 9 green, 1 blue, 12 red; 1 blue, 18 green, 8 red; 2 blue, 6 green, 13 red; 3 blue, 13 red, 7 green; 5 blue, 4 red, 4 green; 6 blue, 7 green, 4 red".to_owned(),
            "Game 3: 5 blue, 9 red, 14 green; 10 green, 3 blue; 11 red, 2 blue, 8 green; 5 red, 2 blue; 5 blue, 7 green, 8 red".to_owned(),
            "Game 4: 2 red, 3 blue, 2 green; 17 green, 6 blue, 1 red; 3 blue, 5 green, 1 red; 4 red, 1 blue, 16 green; 5 red, 4 blue, 13 green; 14 green, 5 blue, 6 red".to_owned(),
            "Game 5: 3 red, 17 green, 10 blue; 9 blue, 5 green; 14 green, 9 blue, 11 red".to_owned(),
        ];
        let mut games: Vec<Game> = Vec::with_capacity(5);
        let mut loader: GameLoader<_, _> = test_data.iter().into();
        while let Some(g) = loader.next_game_fast() {
            games.push(g.unwrap());
        }
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
