use std::{cmp::Ordering, collections::HashMap};

use parsers::parse_input;

advent_of_code::solution!(7);

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Card {
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    /// Compares two cards, with J as Joker being the lowest card (instead of as Jack)
    fn joker_cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::J, Self::J) => Ordering::Equal,
            (Self::J, _) => Ordering::Less,
            (_, Self::J) => Ordering::Greater,
            _ => self.cmp(other),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeKind = 3,
    FullHouse = 4,
    FourKind = 5,
    FiveKind = 6,
}

pub type Game = (Hand, u32);

#[derive(Debug)]
pub struct Hand {
    map: HashMap<Card, i8>,
    vec: Vec<Card>,

    cached_type: Option<HandType>,
}

impl Hand {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            vec: Vec::with_capacity(5),
            cached_type: None,
        }
    }

    fn get_type(&self) -> HandType {
        //
        // if self.cached_type.is_some() {
        //     return self.cached_type.as_ref().unwrap();
        // }

        // check for each type of hand, from top down
        let card_types = self.map.keys().len();

        let hand_type = match card_types {
            1 => Some(HandType::FiveKind),
            2 => match self.map.values().collect::<Vec<&i8>>().first().unwrap() {
                1 | 4 => Some(HandType::FourKind),
                2 | 3 => Some(HandType::FullHouse),
                _ => panic!("Unable to determine hand (Branch 2)"),
            },
            3 => {
                // either threekind or twopair
                if self.map.values().collect::<Vec<&i8>>().contains(&&2) {
                    Some(HandType::TwoPair)
                } else {
                    Some(HandType::ThreeKind)
                }
            }
            4 => Some(HandType::OnePair),
            5 => Some(HandType::HighCard),
            _ => panic!("Unable to determine hand"),
        };

        hand_type.unwrap()
    }

    fn joker_get_type(&self) -> HandType {
        // check for each type of hand, from top down
        let card_types = self.map.keys().len();

        let mut hand_type = match card_types {
            1 => HandType::FiveKind,
            2 => match self.map.values().collect::<Vec<&i8>>().first().unwrap() {
                1 | 4 => HandType::FourKind,
                2 | 3 => HandType::FullHouse,
                _ => panic!("Unable to determine hand (Branch 2)"),
            },
            3 => {
                // either threekind or twopair
                if self.map.values().collect::<Vec<&i8>>().contains(&&2) {
                    HandType::TwoPair
                } else {
                    HandType::ThreeKind
                }
            }
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => panic!("Unable to determine hand"),
        };

        // count how many jokers, then increase hand type by that number
        let joker_count = self.map.get(&Card::J).unwrap_or(&0);

        if joker_count > &0 {
            hand_type = match hand_type as i8 + joker_count {
                0 => HandType::HighCard,
                1 => HandType::OnePair,
                2 => HandType::TwoPair,
                3 => HandType::ThreeKind,
                4 | 5 => HandType::FourKind, // maybe a bit of a cheat, but jokers will always prefer fourkind over fullhouse
                _ => HandType::FiveKind,
            }
        }

        hand_type
    }
}

fn char_to_card(c: &char) -> Option<Card> {
    match c {
        'A' => Some(Card::A),
        'K' => Some(Card::K),
        'Q' => Some(Card::Q),
        'J' => Some(Card::J),
        'T' => Some(Card::T),
        '9' => Some(Card::N9),
        '8' => Some(Card::N8),
        '7' => Some(Card::N7),
        '6' => Some(Card::N6),
        '5' => Some(Card::N5),
        '4' => Some(Card::N4),
        '3' => Some(Card::N3),
        '2' => Some(Card::N2),
        _ => None,
    }
}

mod parsers {
    use super::*;

    use nom::{
        character::complete::{anychar, line_ending, space1, u32},
        combinator::map,
        multi::{fold_many_m_n, separated_list1},
        sequence::separated_pair,
        IResult,
    };

    pub fn hand_parser(input: &str) -> IResult<&str, Hand> {
        fold_many_m_n(
            5,
            5,
            map(anychar, |c| char_to_card(&c).unwrap()),
            Hand::new,
            |mut hand, el| {
                if let Some(x) = hand.map.get_mut(&el) {
                    *x += 1;
                } else {
                    hand.map.insert(el, 1);
                }
                hand.vec.push(el);
                hand
            },
        )(input)
    }

    pub fn parse_input(input: &str) -> IResult<&str, Vec<Game>> {
        separated_list1(line_ending, separated_pair(hand_parser, space1, u32))(input)
    }
}

fn hand_sorter((a, _): &Game, (b, _): &Game) -> Ordering {
    let a_type = a.cached_type.as_ref().unwrap();
    let b_type = b.cached_type.as_ref().unwrap();
    match a_type.cmp(b_type) {
        Ordering::Equal => {
            // compare each card from left to right
            // assume both vecs are same size
            for (a, b) in a.vec.iter().zip(b.vec.iter()) {
                match a.cmp(b) {
                    Ordering::Equal => continue,
                    Ordering::Less => return Ordering::Less,
                    Ordering::Greater => return Ordering::Greater,
                }
            }

            // realistically should never happen
            Ordering::Equal
        }
        Ordering::Less => Ordering::Less,
        Ordering::Greater => Ordering::Greater,
    }
}

fn joker_hand_sorter((a, _): &Game, (b, _): &Game) -> Ordering {
    let a_type = a.cached_type.as_ref().unwrap();
    let b_type = b.cached_type.as_ref().unwrap();

    match a_type.cmp(b_type) {
        Ordering::Equal => {
            // compare each card from left to right
            // assume both vecs are same size
            for (a, b) in a.vec.iter().zip(b.vec.iter()) {
                match a.joker_cmp(b) {
                    Ordering::Equal => continue,
                    Ordering::Less => {
                        // println!("\tOrdering: {:?}", Ordering::Less);
                        return Ordering::Less;
                    }
                    Ordering::Greater => {
                        // println!("\tOrdering: {:?}", Ordering::Less);
                        return Ordering::Greater;
                    }
                }
            }

            // realistically should never happen with our input
            Ordering::Equal
        }
        Ordering::Less => Ordering::Less,
        Ordering::Greater => Ordering::Greater,
    }
}

#[allow(unused_variables)]
#[allow(unused_must_use)]
pub fn part_one(input: &str) -> Option<u32> {
    let (_, mut games) = parse_input(input).unwrap();

    games.iter_mut().for_each(|(hand, _)| {
        hand.cached_type = Some(hand.get_type());
    });

    games.sort_by(hand_sorter);

    Some(
        games
            .iter()
            .enumerate()
            .fold(0_u32, |acc, (i, (_, bet))| acc + (i as u32 + 1) * bet),
    )
}

#[allow(unused_variables)]
#[allow(unused_must_use)]
pub fn part_two(input: &str) -> Option<u32> {
    let (_, mut games) = parse_input(input).unwrap();

    games.iter_mut().for_each(|(hand, _)| {
        hand.cached_type = Some(hand.joker_get_type());
    });

    games.sort_by(joker_hand_sorter);

    Some(
        games
            .iter()
            .enumerate()
            .fold(0_u32, |acc, (i, (_, bet))| acc + (i as u32 + 1) * bet),
    )
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    use super::HandType::*;

    const EXAMPLE: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    #[test]
    fn test_part_one() {
        let result = part_one(EXAMPLE);

        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(EXAMPLE);

        assert_eq!(result, Some(5905));
    }

    #[rstest]
    #[case("32T3K", OnePair)]
    #[case("T55J5", FourKind)]
    #[case("8234J", OnePair)]
    #[case("JJJJJ", FiveKind)]
    #[case("KKKKJ", FiveKind)]
    #[case("KKKJJ", FiveKind)]
    #[case("KKJJJ", FiveKind)]
    #[case("KJJJJ", FiveKind)]
    #[case("KKQJJ", FourKind)]
    fn test_joker_type(#[case] hand: &str, #[case] expected: HandType) {
        let (_, mut hand) = parsers::hand_parser(hand).unwrap();
        hand.cached_type = Some(hand.joker_get_type());
        assert_eq!(hand.cached_type.unwrap(), expected);
    }

    #[rstest]
    #[case("J8KKK", "J8KKK", Ordering::Equal)]
    #[case("J2345", "J3245", Ordering::Less)]
    #[case("JJ345", "J3J45", Ordering::Less)]
    #[case("JKKK2", "QQQQ2", Ordering::Less)]
    #[case("KKKKJ", "2222J", Ordering::Greater)]
    #[case("JJJ4J", "J4JJJ", Ordering::Less)]
    fn test_joker_sorting(#[case] a: &str, #[case] b: &str, #[case] expected: Ordering) {
        let (_, mut a) = parsers::hand_parser(a).unwrap();
        let (_, mut b) = parsers::hand_parser(b).unwrap();
        a.cached_type = Some(a.joker_get_type());
        b.cached_type = Some(b.joker_get_type());
        assert_eq!(joker_hand_sorter(&(a, 0), &(b, 0)), expected);
    }
}
