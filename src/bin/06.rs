use std::{
    iter::{zip, Zip},
    vec::IntoIter,
};

use parsers::parse_input;

advent_of_code::solution!(6);

mod parsers {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{digit1, line_ending, space0},
        combinator::map,
        multi::{fold_many1, many1},
        sequence::{delimited, preceded, separated_pair},
        IResult,
    };

    fn parse_u64(input: &str) -> IResult<&str, u64> {
        digit1(input).map(|(input, value)| (input, value.parse().unwrap()))
    }

    fn nums(input: &str) -> IResult<&str, Vec<u64>> {
        many1(delimited(space0, parse_u64, alt((space0, line_ending))))(input)
    }

    pub fn parse_input(input: &str) -> IResult<&str, (Vec<u64>, Vec<u64>)> {
        separated_pair(
            preceded(tag("Time:"), nums),
            line_ending,
            preceded(tag("Distance:"), nums),
        )(input)
    }

    fn whole_num_u64(input: &str) -> IResult<&str, u64> {
        map(
            fold_many1(
                delimited(space0, digit1, alt((space0, line_ending))),
                String::new,
                |mut acc, dig| {
                    acc.push_str(dig);
                    acc
                },
            ),
            |s| s.parse().unwrap(),
        )(input)
    }

    pub fn parse_input_2(input: &str) -> IResult<&str, (u64, u64)> {
        separated_pair(
            preceded(tag("Time:"), whole_num_u64),
            line_ending,
            preceded(tag("Distance:"), whole_num_u64),
        )(input)
    }
}

fn prepare(input: &str) -> Zip<IntoIter<u64>, IntoIter<u64>> {
    let (_, (time, dist)) = parse_input(input).unwrap();

    zip(time, dist)
}

fn is_winning(hold: &u64, time: &u64, dist: &u64) -> bool {
    let remain = time - hold;

    &(hold * remain) > dist
}

pub fn part_one(input: &str) -> Option<u64> {
    let races = prepare(input);

    // brute force approach
    let result = races
        .map(|race| {
            // convert race into # of ways to win
            let (time, dist) = race;

            // 0 & race.time are guaranteed losses

            let mut wins: u64 = 0;
            // hand roll for loop for short-circuiting
            for hold in 1..time {
                if is_winning(&hold, &time, &dist) {
                    wins += 1
                } else if wins > 0 {
                    break;
                }
            }

            wins
        })
        .product();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, (time, dist)) = parsers::parse_input_2(input).unwrap();

    // quadratic formula solution
    // fastest, but can't be applied 1:1 to example data (smaller numbers), might still implement a binary search
    // solution for both

    let a = 1.;
    let b = 0.0 - time as f64;
    let c = dist as f64;

    let x = ((0. - b) - (b.powf(2.) - (4. * a * c)).sqrt()) / (2.0 * a);
    let y = ((0. - b) + (b.powf(2.) - (4. * a * c)).sqrt()) / (2.0 * a);

    Some((y.floor() as u64 + 1) - (x.ceil() as u64))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_part_one() {
        let result = part_one(EXAMPLE);
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(EXAMPLE);
        assert_eq!(result, Some(71503));
    }
}
