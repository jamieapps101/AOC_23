pub trait TextToNumberRev<S: Iterator<Item = char>> {
    fn text_to_number_rev(self) -> TextToNumberConverterRev<S>;
}

impl<S: Iterator<Item = char>> TextToNumberRev<S> for S {
    fn text_to_number_rev(self) -> TextToNumberConverterRev<S> {
        TextToNumberConverterRev {
            source: self,
            over_flow_buffer: Vec::with_capacity(10),
            process_buffer: Vec::with_capacity(10),
            out_index: 0,
        }
    }
}

impl<S: Iterator<Item = char>> Iterator for TextToNumberConverterRev<S> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        // dbg!("call next");
        self.get_next_processed_char()
    }
}

pub struct TextToNumberConverterRev<S: Iterator<Item = char>> {
    source: S,
    over_flow_buffer: Vec<char>,
    process_buffer: Vec<char>,
    out_index: usize,
}

impl<S: Iterator<Item = char>> TextToNumberConverterRev<S> {
    fn reset_buffer(&mut self) {
        self.over_flow_buffer.clear();
        self.out_index = 0;
    }
    fn get_next_char(&mut self) -> Option<char> {
        if self.out_index < self.over_flow_buffer.len() {
            let next_char = self.over_flow_buffer[self.out_index];
            self.out_index += 1;
            if self.out_index == self.over_flow_buffer.len() {
                self.reset_buffer();
            }
            Some(next_char)
        } else {
            self.source.next()
        }
    }

    fn get_next_processed_char(&mut self) -> Option<char> {
        if self.process_buffer.is_empty() {
            // dbg!("buffer is empty");
            let c_0 = self.get_next_char()?;
            // dbg!(&c_0);
            self.process_buffer.push(c_0);
            return self.get_next_processed_char();
        }

        // dbg!(self.process_buffer.len());
        match self.process_buffer.len() {
            0 => unreachable!(),
            1 => {
                // dbg!("len 1");
                // dbg!(self.process_buffer[0]);
                match self.process_buffer[0] {
                    'e' | 'o' | 'x' | 'r' | 'n' | 't' => {
                        // dbg!("matched");
                        // add a char to buffer
                        // call again
                        if let Some(c) = self.get_next_char() {
                            self.process_buffer.push(c);
                            return self.get_next_processed_char();
                        }
                    }
                    _ => {
                        // dbg!("not matched");
                    }
                }
                // dbg!("return first character");
                let t = self.process_buffer[0];
                self.process_buffer.clear();
                Some(t)
            }
            2 => {
                match &self.process_buffer[..] {
                    &['e', 'n']
                    | &['o', 'w']
                    | &['o', 'r']
                    | &['x', 'i']
                    | &['r', 'u']
                    | &['e', 'v']
                    | &['e', 'e']
                    | &['n', 'e']
                    | &['t', 'h'] => {
                        // add a char to buffer
                        // call again
                        if let Some(c) = self.get_next_char() {
                            self.process_buffer.push(c);
                            return self.get_next_processed_char();
                        }
                    }
                    _ => {}
                }
                // if no other chat
                // move end chars to overflow buffer
                self.over_flow_buffer.extend(self.process_buffer.drain(1..));
                let t = self.process_buffer[0];
                self.process_buffer.clear();
                Some(t)
            }
            3 => {
                match &self.process_buffer[..] {
                    // explictly match 3 letter numbers
                    &['e', 'n', 'o'] => {
                        self.process_buffer.clear();
                        return Some('1');
                    }
                    &['o', 'w', 't'] => {
                        self.process_buffer.clear();
                        return Some('2');
                    }
                    &['x', 'i', 's'] => {
                        self.process_buffer.clear();
                        return Some('6');
                    }
                    &['r', 'u', 'o']
                    | &['e', 'v', 'i']
                    | &['e', 'n', 'i']
                    | &['e', 'e', 'r']
                    | &['n', 'e', 'v']
                    | &['t', 'h', 'g']
                    | &['o', 'r', 'e'] => {
                        // add a char to buffer
                        // call again
                        if let Some(c) = self.get_next_char() {
                            self.process_buffer.push(c);
                            return self.get_next_processed_char();
                        }
                    }
                    _ => {}
                }
                // if no other chat
                // move end chars to overflow buffer
                self.over_flow_buffer.extend(self.process_buffer.drain(1..));
                let t = self.process_buffer[0];
                self.process_buffer.clear();
                Some(t)
            }

            4 => {
                match self.process_buffer[..] {
                    // explictly match 4 letter numbers
                    ['e', 'v', 'i', 'f'] => {
                        self.process_buffer.clear();
                        return Some('5');
                    }
                    ['r', 'u', 'o', 'f'] => {
                        self.process_buffer.clear();
                        return Some('4');
                    }
                    ['e', 'n', 'i', 'n'] => {
                        self.process_buffer.clear();
                        return Some('9');
                    }
                    ['o', 'r', 'e', 'z'] => {
                        self.process_buffer.clear();
                        return Some('0');
                    }
                    ['e', 'e', 'r', 'h'] | ['n', 'e', 'v', 'e'] | ['t', 'h', 'g', 'i'] => {
                        // add a char to buffer
                        // call again
                        if let Some(c) = self.get_next_char() {
                            self.process_buffer.push(c);
                            return self.get_next_processed_char();
                        }
                    }
                    _ => {}
                }
                // if no other chat
                // move end chars to overflow buffer
                self.over_flow_buffer.extend(self.process_buffer.drain(1..));
                let t = self.process_buffer[0];
                self.process_buffer.clear();
                Some(t)
            }

            5 => {
                match self.process_buffer[..] {
                    // explictly match 5 letter numbers
                    ['t', 'h', 'g', 'i', 'e'] => {
                        self.process_buffer.clear();
                        return Some('8');
                    }
                    ['n', 'e', 'v', 'e', 's'] => {
                        self.process_buffer.clear();
                        return Some('7');
                    }
                    ['e', 'e', 'r', 'h', 't'] => {
                        self.process_buffer.clear();
                        return Some('3');
                    }
                    _ => {}
                }
                // if no other chat
                // move end chars to overflow buffer
                self.over_flow_buffer.extend(self.process_buffer.drain(1..));
                let t = self.process_buffer[0];
                self.process_buffer.clear();
                Some(t)
            }
            _ => panic!("Should not reach here"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_words_to_numbers() {
        let s = "one";
        let s_chars = s.chars().rev();
        let n_chars = s_chars.text_to_number_rev().collect::<String>();
        assert_eq!(n_chars.as_str(), "1");

        let s = "oneabc2two";
        // let s = "oneabc2twoweothree,poedsfourx29fivexpmeisixoimseveneight ninezero";
        let s_chars = s.chars().rev();
        let n_chars = s_chars.text_to_number_rev().collect::<String>();
        assert_eq!(n_chars.as_str(), "22cba1");
        // assert_eq!(n_chars.as_str(), "1abc22weo3,poeds4x295xpmei6oim78 90");

        let s = "oneabc2twoweothree,poeds";
        let s_chars = s.chars().rev();
        let n_chars = s_chars.text_to_number_rev().collect::<String>();
        assert_eq!(n_chars.as_str(), "sdeop,3oew22cba1");
        // assert_eq!(n_chars.as_str(), "1abc22weo3,poeds");

        let s = "oneabc2twoweothree,poedsfourx29fivexpmeisixoimseveneight ninezero";
        let s_chars = s.chars().rev();
        let n_chars = s_chars.text_to_number_rev().collect::<String>();
        assert_eq!(n_chars.as_str(), "09 87mio6iempx592x4sdeop,3oew22cba1");
        // assert_eq!(n_chars.as_str(), "1abc22weo3,poeds4x295xpmei6oim78 90");
    }
}
