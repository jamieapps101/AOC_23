#[derive(Debug, PartialEq, Copy, Clone, Eq, PartialOrd, Ord)]
enum Card {
    Ace = 13,
    King = 12,
    Queen = 11,
    N10 = 9,
    N9 = 8,
    N8 = 7,
    N7 = 6,
    N6 = 5,
    N5 = 4,
    N4 = 3,
    N3 = 2,
    N2 = 1,
    Jack = 0,
}

#[derive(PartialEq, Debug)]
enum CardParseError {
    UnexpectedChar(char),
}

impl TryFrom<char> for Card {
    type Error = CardParseError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase() {
            'a' => Ok(Card::Ace),
            'k' => Ok(Card::King),
            'q' => Ok(Card::Queen),
            'j' => Ok(Card::Jack),
            't' => Ok(Card::N10),
            '9' => Ok(Card::N9),
            '8' => Ok(Card::N8),
            '7' => Ok(Card::N7),
            '6' => Ok(Card::N6),
            '5' => Ok(Card::N5),
            '4' => Ok(Card::N4),
            '3' => Ok(Card::N3),
            '2' => Ok(Card::N2),
            _ => Err(CardParseError::UnexpectedChar(value)),
        }
    }
}

#[derive(PartialEq, Debug)]
struct Hand {
    cards: [Card; 5],
}

#[derive(PartialEq, Debug)]
enum HandParseError {
    TooShort,
    TooLong,
    CardParseError(CardParseError),
}

impl TryFrom<&str> for Hand {
    type Error = HandParseError;
    fn try_from(s: &str) -> Result<Hand, Self::Error> {
        let mut cards = [Card::Ace; 5];
        match s.len().partial_cmp(&5).unwrap() {
            Ordering::Greater => return Err(HandParseError::TooLong),
            Ordering::Less => return Err(HandParseError::TooShort),
            _ => {}
        }
        for (i, c) in s.chars().enumerate() {
            cards[i] = c.try_into().map_err(HandParseError::CardParseError)?;
        }
        Ok(Self { cards })
    }
}

#[derive(PartialEq, Debug)]
#[allow(dead_code)]
enum HandCompare {
    Wins,
    Loses,
    Draws,
}

use std::cmp::Ordering;

impl Hand {
    fn against(&self, other: &Self) -> HandCompare {
        let self_type = HandType::best(self);
        let other_type = HandType::best(other);
        match self_type.partial_cmp(&other_type).unwrap() {
            Ordering::Less => HandCompare::Loses,
            Ordering::Greater => HandCompare::Wins,
            Ordering::Equal => {
                for i in 0..5 {
                    match self.cards[i].partial_cmp(&other.cards[i]).unwrap() {
                        Ordering::Less => return HandCompare::Loses,
                        Ordering::Greater => return HandCompare::Wins,
                        Ordering::Equal => {}
                    }
                }
                panic!("We should not get here");
                // HandCompare::Draws
            }
        }
    }
}

#[derive(PartialEq, Debug)]
enum HandType {
    FiveOfAKind(Card),
    FourOfAKind(Card),
    FullHouse(Card, Card),
    ThreeOfAKind(Card),
    TwoPair(Card, Card),
    OnePair(Card),
    HighCard(Card),
}

impl HandType {
    fn value(&self) -> u32 {
        match self {
            HandType::FiveOfAKind(_) => 6,
            HandType::FourOfAKind(_) => 5,
            HandType::FullHouse(_, _) => 4,
            HandType::ThreeOfAKind(_) => 3,
            HandType::TwoPair(_, _) => 2,
            HandType::OnePair(_) => 1,
            HandType::HighCard(_) => 0,
        }
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value().partial_cmp(&other.value())
    }
}

impl From<Hand> for HandType {
    fn from(hand: Hand) -> Self {
        (&hand).into()
    }
}

impl HandType {
    fn best(hand: &Hand) -> Self {
        let joker_count = hand.cards.iter().filter(|&c| *c == Card::Jack).count();
        let cards = ['2', '3', '4', '5', '6', '7', '8', '9', 't', 'q', 'k', 'a'];
        let mut counts = cards
            .into_iter()
            .map(|c| Card::try_from(c).unwrap())
            .map(|c| (c, hand.cards.into_iter().filter(|&h_c| h_c == c).count()))
            .filter(|(_card, count)| count > &0)
            .collect::<Vec<(Card, usize)>>();
        counts.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // dbg!(&counts);
        assert_eq!(counts.iter().map(|c| c.1).sum::<usize>() + joker_count, 5);

        return match counts.len() {
            // if there is a single card type, then a five-of-a-kind will always be possible
            0 | 1 => Self::FiveOfAKind(Card::Jack),

            2 => {
                if counts[0].1 + joker_count == 4 {
                    Self::FourOfAKind(Card::Jack)
                } else {
                    Self::FullHouse(Card::Jack, Card::Jack)
                }
                // if there are only two card types, then a full house will always be possible
            }
            3 => {
                if counts[0].1 + joker_count == 3 {
                    Self::ThreeOfAKind(Card::Jack)
                } else {
                    Self::TwoPair(Card::Jack, Card::Jack)
                }
            }
            4 => Self::OnePair(Card::Jack),

            5 => Self::HighCard(Card::Jack),

            _ => panic!("Hit {} counts ({} jokers)", counts.len(), joker_count),
        };
    }
}

impl From<&Hand> for HandType {
    fn from(hand: &Hand) -> Self {
        let mut cards = Vec::from(hand.cards);
        cards.sort();
        let mut last_card = None;
        let mut current_count = 0;

        let mut max_count = 0;
        let mut max_card = None;
        let mut second_max_count = 0;
        let mut second_max_card = None;

        let mut high_card = None;

        let mut card_iter = cards.into_iter();
        loop {
            match (card_iter.next(), last_card) {
                (Some(c), Some(last_c)) => {
                    if c > high_card.unwrap() {
                        high_card = Some(c);
                    }
                    if c == last_c {
                        // current character is the same as the last
                        current_count += 1;
                    } else {
                        // current character is different from last
                        if current_count >= max_count {
                            second_max_count = max_count;
                            second_max_card = max_card;
                            max_count = current_count;
                            max_card = Some(last_c);
                        } else if current_count >= second_max_count {
                            second_max_count = current_count;
                            second_max_card = Some(last_c);
                        }
                        current_count = 1;
                        last_card = Some(c);
                    }
                }
                (Some(c), None) => {
                    // on the first character
                    high_card = Some(c);
                    last_card = Some(c);
                    current_count = 1;
                }
                (None, Some(last_c)) => {
                    // had the last character
                    if current_count >= max_count {
                        second_max_count = max_count;
                        second_max_card = max_card;
                        max_count = current_count;
                        max_card = Some(last_c);
                    } else if current_count >= second_max_count {
                        second_max_count = current_count;
                        second_max_card = Some(last_c);
                    }
                    break;
                }
                (None, None) => unreachable!(),
            }
        }

        // dbg!(max_count);
        // dbg!(second_max_count);
        match (max_count, second_max_count) {
            (5, 0) => HandType::FiveOfAKind(max_card.unwrap()),
            (4, 1) => HandType::FourOfAKind(max_card.unwrap()),
            (3, 2) => HandType::FullHouse(max_card.unwrap(), second_max_card.unwrap()),
            (3, 0..=1) => HandType::ThreeOfAKind(max_card.unwrap()),
            (2, 2) => HandType::TwoPair(max_card.unwrap(), second_max_card.unwrap()),
            (2, 0..=1) => HandType::OnePair(max_card.unwrap()),
            (1, 1) => HandType::HighCard(high_card.unwrap()),
            (mc, cc) => panic!("Hit undefined state ({},{})", mc, cc),
        }
    }
}

fn calculate_set_score(mut set: Vec<(Hand, u32)>) -> u32 {
    set.sort_by(|a, b| match a.0.against(&b.0) {
        HandCompare::Wins => Ordering::Greater,
        HandCompare::Loses => Ordering::Less,
        HandCompare::Draws => unreachable!(),
    });
    // set.iter().for_each(|i| {
    //     println!("{:?} {:?}", HandType::from(&i.0), i);
    // });
    // dbg!(&set);
    set.into_iter()
        .enumerate()
        .map(|(index, (_hand, bid))| (index as u32 + 1) * bid)
        .sum()
}

fn parse_strings<E, I: Iterator<Item = Result<String, E>>>(source: I) -> Vec<(Hand, u32)> {
    source
        .map_while(Result::ok)
        .filter(|s| !s.is_empty())
        .map(|s| {
            (
                Hand::try_from(&s[0..5]).unwrap(),
                s[6..].parse::<u32>().unwrap(),
            )
        })
        .collect()
}

#[cfg(not(tarpaulin_include))]
fn main() {
    let hand_bids = parse_strings(std::io::stdin().lines());

    println!("loaded: {} hands", hand_bids.len());
    let total_score = calculate_set_score(hand_bids);

    println!("total_score: {total_score}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_best() {
        assert_eq!(
            HandType::best(&Hand::try_from("QJJQ2").unwrap()),
            HandType::FourOfAKind(Card::Jack),
        );

        let a = Hand::try_from("JKKK2").unwrap();
        let b = Hand::try_from("QQQQ2").unwrap();
        assert_eq!(HandType::best(&a), HandType::FourOfAKind(Card::Jack));
        assert_eq!(HandType::best(&b), HandType::FourOfAKind(Card::Jack));
        assert_eq!(a.against(&b), HandCompare::Loses);
    }

    #[test]
    fn test_card_conversion() {
        assert_eq!(Card::try_from('a'), Ok(Card::Ace));
        assert_eq!(Card::try_from('k'), Ok(Card::King));
        assert_eq!(Card::try_from('q'), Ok(Card::Queen));
        assert_eq!(Card::try_from('j'), Ok(Card::Jack));
        assert_eq!(Card::try_from('t'), Ok(Card::N10));
        assert_eq!(Card::try_from('9'), Ok(Card::N9));
        assert_eq!(Card::try_from('8'), Ok(Card::N8));
        assert_eq!(Card::try_from('7'), Ok(Card::N7));
        assert_eq!(Card::try_from('6'), Ok(Card::N6));
        assert_eq!(Card::try_from('5'), Ok(Card::N5));
        assert_eq!(Card::try_from('4'), Ok(Card::N4));
        assert_eq!(Card::try_from('3'), Ok(Card::N3));
        assert_eq!(Card::try_from('2'), Ok(Card::N2));
        assert_eq!(
            Card::try_from('?'),
            Err(CardParseError::UnexpectedChar('?'))
        );
    }

    #[test]
    #[ignore] // ignore this test as it requires files not checked in
    fn test_parse_strings() {
        let file = std::fs::File::open("data/test_input.txt").unwrap();
        let buf_reader = std::io::BufReader::new(file);
        use std::io::BufRead;
        let out = parse_strings(buf_reader.lines());
        let reference = vec![
            (Hand::try_from("32T3K").unwrap(), 765),
            (Hand::try_from("T55J5").unwrap(), 684),
            (Hand::try_from("KK677").unwrap(), 28),
            (Hand::try_from("KTJJT").unwrap(), 220),
            (Hand::try_from("QQQJA").unwrap(), 483),
        ];
        assert_eq!(out, reference);
    }

    #[test]
    fn test_parse_hand() {
        assert_eq!(
            Hand::try_from("33332"),
            Ok(Hand {
                cards: [Card::N3, Card::N3, Card::N3, Card::N3, Card::N2]
            })
        );
        assert_eq!(Hand::try_from("333333"), Err(HandParseError::TooLong));
        assert_eq!(Hand::try_from("333"), Err(HandParseError::TooShort));
        // 2AAAA
    }

    #[test]
    fn test_against() {
        let a = Hand::try_from("33332").unwrap();
        let b = Hand::try_from("2AAAA").unwrap();
        assert_eq!(a.against(&b), HandCompare::Wins);
    }

    mod hand_type {
        use super::*;

        #[test]
        fn test_five_of_kind() {
            // five of a kind
            assert_eq!(
                Hand::try_from("AAAAA").unwrap().try_into(),
                Ok(HandType::FiveOfAKind(Card::Ace))
            );
        }

        #[test]
        fn test_four_of_kind() {
            // four of a kind
            for i in 0..5 {
                let mut test_string = String::with_capacity(5);
                for j in 0..5 {
                    if i == j {
                        test_string.push('2');
                    } else {
                        test_string.push('A');
                    }
                }
                dbg!(test_string.as_str());
                assert_eq!(
                    Hand::try_from(test_string.as_str()).unwrap().try_into(),
                    Ok(HandType::FourOfAKind(Card::Ace))
                );
            }
        }

        #[test]
        fn test_full_house() {
            let pair_position_picker = (0..5)
                .into_iter()
                .map(|i| (0..5).into_iter().map(move |j| (i, j)))
                .flatten()
                .filter(|(i, j)| i != j);
            for (a, b) in pair_position_picker {
                let mut test_string = String::with_capacity(5);
                for i in 0..5 {
                    if i == a || i == b {
                        test_string.push('a');
                    } else {
                        test_string.push('2');
                    }
                }
                dbg!(test_string.as_str());
                assert_eq!(
                    Hand::try_from(test_string.as_str()).unwrap().try_into(),
                    Ok(HandType::FullHouse(Card::N2, Card::Ace))
                );
            }
        }

        #[test]
        fn test_three_of_kind() {
            // three of a kind
            for i in 0..5 {
                for j in 0..5 {
                    if i == j {
                        continue;
                    }
                    let mut test_string = String::with_capacity(5);
                    for k in 0..5 {
                        if k == i {
                            test_string.push('2');
                        } else if k == j {
                            test_string.push('3');
                        } else {
                            test_string.push('A');
                        }
                    }
                    dbg!(test_string.as_str());
                    assert_eq!(
                        Hand::try_from(test_string.as_str()).unwrap().try_into(),
                        Ok(HandType::ThreeOfAKind(Card::Ace))
                    );
                }
            }
        }

        #[test]
        fn test_two_pair() {
            let pair_position_picker = (0..5)
                .into_iter()
                .map(|i| (0..5).into_iter().map(move |j| (i, j)))
                .flatten()
                .filter(|(i, j)| i != j);
            let pos_iter = pair_position_picker
                .clone()
                .map(|a| pair_position_picker.clone().map(move |b| (a, b)))
                .flatten()
                .filter(|((a0, a1), (b0, b1))| a0 != b0 && a1 != b1 && a0 != b1 && a1 != b0);

            dbg!("running");
            let mut combinations = 0;
            for ((a0, a1), (b0, b1)) in pos_iter {
                // dbg!(((a0, a1), (b0, b1)));
                let mut test_str = String::with_capacity(5);
                for i in 0..5 {
                    if i == a0 || i == a1 {
                        test_str.push('2');
                    } else if i == b0 || i == b1 {
                        test_str.push('3');
                    } else {
                        test_str.push('a');
                    }
                }
                dbg!(test_str.as_str());
                assert_eq!(
                    Hand::try_from(test_str.as_str()).unwrap().try_into(),
                    Ok(HandType::TwoPair(Card::N3, Card::N2))
                );
                combinations += 1;
            }
            println!("tested {} combinations", combinations);
        }

        #[test]
        fn test_one_pair() {
            let pair_position_picker = (0..5)
                .into_iter()
                .map(|i| (0..5).into_iter().map(move |j| (i, j)))
                .flatten()
                .filter(|(i, j)| i != j);
            let opts = ['2', '3', '4', '5', '6'];
            for (a, b) in pair_position_picker {
                let mut test_string = String::with_capacity(5);
                for i in 0..5 {
                    if i == a || i == b {
                        test_string.push('a');
                    } else {
                        test_string.push(opts[i]);
                    }
                }
                dbg!(test_string.as_str());
                assert_eq!(
                    Hand::try_from(test_string.as_str()).unwrap().try_into(),
                    Ok(HandType::OnePair(Card::Ace))
                );
            }
        }

        #[test]
        fn test_high_card() {
            let opts = ['2', '3', '4', '5', '6'];
            for i in 0..5 {
                let mut test_string = String::with_capacity(5);

                for j in 0..5 {
                    if i == j {
                        test_string.push('a');
                    } else {
                        test_string.push(opts[j]);
                    }
                }
                dbg!(test_string.as_str());
                assert_eq!(
                    Hand::try_from(test_string.as_str()).unwrap().try_into(),
                    Ok(HandType::HighCard(Card::Ace))
                );
            }
        }
    }
    #[test]
    fn test_calculate_set_score() {
        let set = vec![
            (Hand::try_from("32T3K").unwrap(), 765),
            (Hand::try_from("T55J5").unwrap(), 684),
            (Hand::try_from("KK677").unwrap(), 28),
            (Hand::try_from("KTJJT").unwrap(), 220),
            (Hand::try_from("QQQJA").unwrap(), 483),
        ];
        assert_eq!((&set[0].0).try_into(), Ok(HandType::OnePair(Card::N3)));
        assert_eq!((&set[1].0).try_into(), Ok(HandType::ThreeOfAKind(Card::N5)));
        assert_eq!(
            (&set[2].0).try_into(),
            Ok(HandType::TwoPair(Card::King, Card::N7))
        );
        assert_eq!(
            (&set[3].0).try_into(),
            Ok(HandType::TwoPair(Card::N10, Card::Jack))
        );
        assert_eq!(
            (&set[4].0).try_into(),
            Ok(HandType::ThreeOfAKind(Card::Queen))
        );
        let score = calculate_set_score(set);
        assert_eq!(score, 5905);
    }

    #[test]
    fn test_calculate_best_score() {
        let set = vec![
            (Hand::try_from("32T3K").unwrap(), 765),
            (Hand::try_from("T55J5").unwrap(), 684),
            (Hand::try_from("KK677").unwrap(), 28),
            (Hand::try_from("KTJJT").unwrap(), 220),
            (Hand::try_from("QQQJA").unwrap(), 483),
        ];
        assert_eq!(HandType::best(&set[0].0), HandType::OnePair(Card::Jack));
        assert_eq!(HandType::best(&set[1].0), HandType::FourOfAKind(Card::Jack));
        assert_eq!(
            HandType::best(&set[2].0),
            HandType::TwoPair(Card::Jack, Card::Jack)
        );
        assert_eq!(HandType::best(&set[3].0), HandType::FourOfAKind(Card::Jack));
        assert_eq!(HandType::best(&set[4].0), HandType::FourOfAKind(Card::Jack));
        let score = calculate_set_score(set);
        assert_eq!(score, 5905);
    }
}
