struct NumberSource<C: Iterator<Item = char>> {
    source: C,
    buffer: String,
    fuse: bool,
}

trait ToNumbers<I: Iterator<Item = char>> {
    fn to_numbers(self) -> NumberSource<I>;
}

impl<I: Iterator<Item = char>> ToNumbers<I> for I {
    fn to_numbers(self) -> NumberSource<I> {
        NumberSource {
            source: self,
            buffer: String::with_capacity(10),
            fuse: false,
        }
    }
}

impl<C: Iterator<Item = char>> NumberSource<C> {
    fn parse_buffer(&mut self) -> Option<u32> {
        if let Ok(v) = self.buffer.parse::<u32>() {
            self.buffer.clear();
            Some(v)
        } else {
            self.fuse = true;
            None
        }
    }
}

impl<C: Iterator<Item = char>> Iterator for NumberSource<C> {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.fuse {
            return None;
        }
        loop {
            if let Some(c) = self.source.next() {
                match (c, self.buffer.is_empty()) {
                    (' ', true) => {
                        // likely parsing space before number, ignore this
                    }
                    (' ', false) => {
                        // space after number, attempt to parse the buffer
                        // and return a value
                        return self.parse_buffer();
                    }
                    ('0'..='9', _) => {
                        self.buffer.push(c);
                    }
                    (_, _) => {
                        self.fuse = true;
                        return None;
                    }
                }
            } else {
                return self.parse_buffer();
            }
        }
    }
}

struct RecordSource<C: Iterator<Item = u32>> {
    time_source: C,
    dist_source: C,
}

use std::str::Chars;

impl<'a> RecordSource<NumberSource<Chars<'a>>> {
    fn new(data: &'a str) -> Self {
        let mut time_source = None;
        let mut dist_source = None;
        for line in data.lines() {
            if let Some(s) = line.strip_prefix("Time:") {
                time_source = Some(s.chars());
            }
            if let Some(s) = line.strip_prefix("Distance:") {
                dist_source = Some(s.chars())
            }
        }
        RecordSource {
            time_source: time_source.unwrap().to_numbers(),
            dist_source: dist_source.unwrap().to_numbers(),
        }
    }
}

impl<C: Iterator<Item = char>> Iterator for RecordSource<NumberSource<C>> {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let (race_time, record_distance) = self.time_source.next().zip(self.dist_source.next())?;
        let mut ways_to_beat_record = 0;
        for charge_t_cand in 0..race_time {
            let travel_t = race_time - charge_t_cand;
            let velocity = charge_t_cand * 1;
            let distance = travel_t * velocity;
            if distance > record_distance {
                ways_to_beat_record += 1;
            }
        }
        Some(ways_to_beat_record)
    }
}

fn main() {
    let data = std::io::stdin()
        .lines()
        .filter_map(Result::ok)
        .filter(|l| !l.is_empty())
        .collect::<Vec<String>>();
    let data = data.join("\n");
    let multipled_score: u32 = RecordSource::new(&data).product();
    println!("multipled_score: {multipled_score}");
}
