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

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
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

    /// Get's the HandType of the Hand. If it hasn't been computed yet, it computes it, and then returns the computed result.
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

    fn hand_parser(input: &str) -> IResult<&str, Hand> {
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
    None
}

#[cfg(test)]
mod tests {
    use super::*;

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

        assert_eq!(result, None);
    }
}
