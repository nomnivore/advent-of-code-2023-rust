use std::collections::HashMap;

advent_of_code::solution!(4);

#[derive(Debug)]
struct Card<'a> {
    id: u32,
    win_nums: HashMap<&'a str, bool>,
    card_nums: Vec<&'a str>,
}

trait Scorable {
    fn points(&self) -> u32;
}

impl<'a> Scorable for Card<'a> {
    fn points(&self) -> u32 {
        self.card_nums
            .iter()
            .filter(|num| self.win_nums.get(*num).is_some())
            .collect::<Vec<&&str>>()
            .len()
            .try_into()
            .unwrap_or(0)
    }
}

fn as_card(line: &str) -> Card {
    let (card, numbers) = line.split_once(": ").unwrap();
    let card_number: u32 = card.strip_prefix("Card ").unwrap().trim().parse().unwrap();

    let (win_str, card_str): (HashMap<&str, bool>, Vec<&str>) = {
        let strs = numbers.split_once(" | ").unwrap();
        (
            strs.0.split_whitespace().map(|key| (key, true)).collect(),
            strs.1.split_whitespace().collect(),
        )
    };

    Card {
        id: card_number,
        win_nums: win_str,
        card_nums: card_str,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(as_card)
            .map(|card| card.points())
            .map(|matches| match matches.checked_sub(1) {
                Some(num) => 2u32.pow(num),
                None => 0,
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut copies: HashMap<u32, u32> = HashMap::new();

    Some(
        input
            .lines()
            .map(as_card)
            .map(|card| {
                // how many copies do we have of this card?
                let card_copies = *copies.get(&(card.id)).unwrap_or(&1);

                // add copies that we won
                for i in 1..card.points() + 1 {
                    let copy = copies.get(&(card.id + i)).unwrap_or(&1);
                    copies.insert(card.id + i, copy + card_copies);
                }

                card_copies
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part_one() {
        let result = part_one(EXAMPLE);
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(EXAMPLE);
        assert_eq!(result, Some(30));
    }
}
