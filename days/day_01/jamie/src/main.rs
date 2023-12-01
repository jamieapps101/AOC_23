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
            while let Some(u) = self.source.next() {
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

fn main() {
    let std_in = std::io::stdin().lines();
    let total: u32 = std_in
        .filter_map(Result::ok)
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.chars()
                .filter(|s| s.is_ascii_digit())
                .first_and_last()
                .collect::<String>()
        })
        .map(|s| s.parse::<u32>().unwrap())
        .sum();
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
}
