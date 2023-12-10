#[derive(Debug, PartialEq, Copy, Clone, Eq, PartialOrd, Ord)]
enum Card {
    Ace = 13,
    King = 12,
    Queen = 11,
    Jack = 10,
    N10 = 9,
    N9 = 8,
    N8 = 7,
    N7 = 6,
    N6 = 5,
    N5 = 4,
    N4 = 3,
    N3 = 2,
    N2 = 1,
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
            cards[i] = c
                .try_into()
                .map_err(|e| HandParseError::CardParseError(e))?;
        }
        Ok(Self { cards })
    }
}

#[derive(PartialEq, Debug)]
enum HandCompare {
    Wins,
    Loses,
    Draws,
}

use std::cmp::Ordering;

impl Hand {
    fn against(&self, other: &Self) -> HandCompare {
        let self_type = HandType::from(self);
        dbg!(&self_type);
        let other_type = HandType::from(other);
        dbg!(&other_type);
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
                return HandCompare::Draws;
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

impl From<&Hand> for HandType {
    fn from(hand: &Hand) -> Self {
        let mut cards = Vec::from(hand.cards);
        cards.sort();
        let mut last = None;
        let mut current = 1;

        let mut max_count = 0;
        let mut max_card = None;
        let mut second_max_count = 0;
        let mut second_max_card = None;

        let mut high_card = None;

        for c in cards.into_iter() {
            dbg!(c);
            if let Some(last_c) = last {
                dbg!(last_c);
                if last_c == c {
                    current += 1;
                } else {
                    current = 1;
                }
                if current > max_count {
                    if last_c != c {
                        second_max_count = max_count;
                        second_max_card = max_card;
                    }
                    max_card = Some(c);
                    max_count = current;
                }
            }
            last = Some(c);
            if let Some(high_c) = high_card {
                if c > high_c {
                    high_card = Some(c);
                }
            } else {
                high_card = Some(c);
            }
        }
        dbg!(max_count);
        dbg!(second_max_count);
        match (max_count, second_max_count) {
            (5, 0) => HandType::FiveOfAKind(max_card.unwrap()),
            (4, _) => HandType::FourOfAKind(max_card.unwrap()),
            (3, 2) => HandType::FullHouse(max_card.unwrap(), second_max_card.unwrap()),
            (3, _) => HandType::ThreeOfAKind(max_card.unwrap()),
            (2, 2) => HandType::TwoPair(max_card.unwrap(), second_max_card.unwrap()),
            (2, _) => HandType::OnePair(max_card.unwrap()),
            (_, _) => HandType::HighCard(high_card.unwrap()),
        }
    }
}

fn calculate_set_score(mut set: Vec<(Hand, u32)>) -> u32 {
    set.sort_by(|a, b| match a.0.against(&b.0) {
        HandCompare::Wins => Ordering::Greater,
        HandCompare::Loses => Ordering::Less,
        HandCompare::Draws => Ordering::Equal,
    });
    dbg!(&set);
    set.into_iter()
        .enumerate()
        .map(|(index, (_hand, bid))| (index as u32 + 1) * bid)
        .sum()
}

fn main() {
    let hand_bids = std::io::stdin()
        .lines()
        .filter_map(Result::ok)
        .filter(|s| !s.is_empty())
        .map(|s| {
            (
                Hand::try_from(&s[0..5]).unwrap(),
                (&s[6..]).parse::<u32>().unwrap(),
            )
        })
        .collect::<Vec<(Hand, u32)>>();
    println!("loaded: {} hands", hand_bids.len());
    let total_score = calculate_set_score(hand_bids);
    println!("total_score: {total_score}");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse_hand() {
        assert_eq!(
            Hand::try_from("33332"),
            Ok(Hand {
                cards: [Card::N3, Card::N3, Card::N3, Card::N3, Card::N2]
            })
        );
        // 2AAAA
    }

    #[test]
    fn test_against() {
        let a = Hand::try_from("33332").unwrap();
        let b = Hand::try_from("2AAAA").unwrap();
        assert_eq!(a.against(&b), HandCompare::Wins);
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
        let score = calculate_set_score(set);
        assert_eq!(score, 6440);
    }
}
