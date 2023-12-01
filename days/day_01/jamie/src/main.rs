mod text_to_number;
use text_to_number::TextToNumber;
mod text_to_number_rev;
use text_to_number_rev::TextToNumberRev;

struct FirstAndLastStruct<T, S> {
    source: S,
    buffer: Option<T>,
}

trait FirstAndLast<T, S> {
    fn first_and_last(self) -> FirstAndLastStruct<T, S>;
}

impl<T, S: Iterator<Item = T>> FirstAndLast<T, S> for S {
    fn first_and_last(self) -> FirstAndLastStruct<T, S> {
        FirstAndLastStruct {
            source: self,
            buffer: None,
        }
    }
}

impl<T: Copy, S: Iterator<Item = T>> Iterator for FirstAndLastStruct<T, S> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buffer.is_some() {
            let mut last: Option<T> = None;
            for u in self.source.by_ref() {
                last = Some(u);
            }
            last.or(std::mem::take(&mut self.buffer))
        } else {
            let item = self.source.next();
            self.buffer = item;
            item
        }
    }
}

fn conv<S: Iterator<Item = String>>(source: S) -> u32 {
    source
        .enumerate()
        .map(|(i, s)| {
            print!("{i}) s = {s} ");
            let first = s
                .chars()
                .text_to_number()
                .filter(|s| s.is_ascii_digit())
                .nth(0);

            let last = s
                .chars()
                .rev()
                .text_to_number_rev()
                .filter(|s| s.is_ascii_digit())
                .nth(0);

            let mut val_string = String::with_capacity(2);
            if let Some(c) = first {
                val_string.push(c);
            }
            if let Some(c) = last {
                val_string.push(c);
            }

            val_string
        })
        .map(|s| {
            let n = s.parse::<u32>().unwrap();
            println!(" ==> {n}");
            n
        })
        .sum()
}

fn main() {
    let std_in = std::io::stdin().lines();
    let total: u32 =
        conv(
            std_in
                .map_while(Result::ok)
                .map_while(|s| if s.is_empty() { None } else { Some(s) }),
        );
    println!("total: {total}");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_first_and_last_iter() {
        let s = "abcdefg";
        let s_chars = s.chars();
        let f_l = s_chars.first_and_last().collect::<String>();
        assert_eq!(f_l.as_str(), "ag");

        let s = "a";
        let s_chars = s.chars();
        let f_l = s_chars.first_and_last().collect::<String>();
        assert_eq!(f_l.as_str(), "aa");
    }

    #[test]
    fn test_first_day() {
        let text = vec![
            "1abc2".to_owned(),
            "pqr3stu8vwx".to_owned(),
            "a1b2c3d4e5f".to_owned(),
            "treb7uchet".to_owned(),
        ];
        let result = conv(text.into_iter());
        assert_eq!(result, 142);
    }

    #[test]
    fn test_second_day() {
        let text = vec![
            "two1nine".to_owned(),
            "eightwothree".to_owned(),
            "abcone2threexyz".to_owned(),
            "xtwone3four".to_owned(),
            "4nineeightseven2".to_owned(),
            "zoneight234".to_owned(),
            "7pqrstsixteen".to_owned(),
        ];
        let result = conv(text.into_iter());
        assert_eq!(result, 281);
    }
}
