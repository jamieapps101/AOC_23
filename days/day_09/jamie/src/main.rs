#[derive(Debug, PartialEq)]
struct Sequence {
    values: Vec<Vec<i32>>,
}

impl<S: Iterator<Item = i32>> From<S> for Sequence {
    fn from(value: S) -> Self {
        let v = value.collect();
        let values = vec![v];
        Self { values }
    }
}

impl Sequence {
    fn analyse(&mut self) {
        while !self.values.last().unwrap().iter().all(|&n| n == 0) {
            let last_vec = self.values.last().unwrap();
            let mut diff_vec = Vec::with_capacity(last_vec.len() - 1);
            for i in 1..last_vec.len() {
                let diff = last_vec[i] - last_vec[i - 1];
                diff_vec.push(diff);
            }
            self.values.push(diff_vec);
        }
    }

    fn extrapolate(&mut self) -> i32 {
        for i in 0..self.values.len() {
            let i_rev = self.values.len() - 1 - i;
            if i_rev == self.values.len() - 1 {
                let next_val = *self.values[i_rev].last().unwrap();
                self.values[i_rev].push(next_val);
            } else {
                let next_val =
                    *self.values[i_rev].last().unwrap() + *self.values[i_rev + 1].last().unwrap();
                self.values[i_rev].push(next_val);
            }
        }
        *self.values[0].last().unwrap()
    }
}

fn main() {
    let res: i32 = std::io::stdin()
        .lines()
        .map_while(Result::ok)
        .filter(|s| !s.is_empty())
        .map(|s| {
            let input = s.split_whitespace().map(|s| s.parse::<i32>().unwrap());
            let mut sequence = Sequence::from(input);
            sequence.analyse();
            sequence.extrapolate()
        })
        .sum();
    println!("res: {res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyse() {
        let input = "0 3 6 9 12 15"
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let mut s = Sequence::from(input.into_iter());
        assert_eq!(
            s,
            Sequence {
                values: vec![vec![0, 3, 6, 9, 12, 15]]
            }
        );
        s.analyse();
        assert_eq!(
            s,
            Sequence {
                values: vec![
                    vec![0, 3, 6, 9, 12, 15],
                    vec![3, 3, 3, 3, 3],
                    vec![0, 0, 0, 0]
                ]
            }
        );
    }

    #[test]
    fn test_extrapolate() {
        let input = "0 3 6 9 12 15"
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let mut s = Sequence::from(input.into_iter());
        s.analyse();
        let next_val = s.extrapolate();
        assert_eq!(
            s,
            Sequence {
                values: vec![
                    vec![0, 3, 6, 9, 12, 15, 18],
                    vec![3, 3, 3, 3, 3, 3],
                    vec![0, 0, 0, 0, 0]
                ]
            }
        );
        assert_eq!(next_val, 18);
    }
}
