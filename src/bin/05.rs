use std::collections::HashMap;

use parsers::parse_input;

advent_of_code::solution!(5);

#[allow(dead_code)]
#[allow(unused_imports)]
mod parsers {
    use super::*;

    use nom::{
        branch::alt,
        bytes::complete::{is_a, tag, take_until, take_while1},
        character::{
            complete::{digit1, line_ending, newline, space0, space1},
            is_alphabetic,
        },
        combinator::{eof, map},
        multi::{count, many0, many1},
        sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
        IResult,
    };

    fn parse_u32(input: &str) -> IResult<&str, u32> {
        digit1(input).map(|(input, value)| (input, value.parse().unwrap()))
    }

    pub fn parse_header(input: &str) -> IResult<&str, Vec<u32>> {
        let numbers = many1(delimited(space0, parse_u32, space0));
        preceded(pair(tag("seeds:"), space1), numbers)(input)
    }

    pub type ChartTitle<'a> = (&'a str, &'a str);
    pub type Mapping = (u32, u32, u32);
    pub type SeedsToPlant = Vec<u32>;
    pub type Chart<'a> = (ChartTitle<'a>, Vec<Mapping>);
    pub fn parse_map(input: &str) -> IResult<&str, (ChartTitle, Vec<Mapping>)> {
        let from = take_until("-");
        let to = take_until(" ");
        let delimited_u32 = |i| delimited(space0, parse_u32, space0)(i);
        pair(
            terminated(
                separated_pair(from, tag("-to-"), to),
                pair(tag(" map:"), line_ending),
            ),
            many1(terminated(
                tuple((delimited_u32, delimited_u32, delimited_u32)),
                alt((line_ending, eof)),
            )),
        )(input)
    }

    pub fn parse_input(input: &str) -> IResult<&str, (SeedsToPlant, Vec<Chart>)> {
        separated_pair(
            parse_header,
            many0(newline),
            many1(delimited(many0(line_ending), parse_map, many0(line_ending))),
        )(input)
    }
}

struct ABChart<'a> {
    from: &'a str,
    to: &'a str,
    mappings: Vec<(u32, u32, u32)>,
}

fn prepare(input: &str) -> (Vec<u32>, HashMap<(&str, &str), ABChart>) {
    let (_, (seeds, charts)) = parse_input(input).unwrap();

    let mut map = HashMap::new();

    charts.into_iter().for_each(|(key, chart)| {
        map.insert(
            key,
            ABChart {
                from: key.0,
                to: key.1,
                mappings: chart,
            },
        );
    });

    (seeds, map)
}

#[allow(unused_variables)]
#[allow(unused_must_use)]
pub fn part_one(input: &str) -> Option<u32> {
    let (seeds, maps) = prepare(input);

    None
}

#[allow(unused_variables)]
#[allow(unused_must_use)]
pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::*;

    #[test]
    fn test_parse_header() {
        let (input, nums) = parse_header(EXAMPLE).unwrap();
        println!("{}", input);
        assert_eq!(nums, [79, 14, 55, 13]);
    }

    #[test]
    fn test_parse_map() {
        let (input, ((from, to), mappings)) = parse_map(
            "seeds-to-soil map:
1 2 3
4 5 6
",
        )
        .unwrap();

        assert_eq!(input, "");
        assert_eq!(from, "seeds");
        assert_eq!(to, "soil");
        assert_eq!(mappings, [(1, 2, 3), (4, 5, 6)]);
    }

    #[test]
    fn test_parse_input() {
        let (input, all) = parse_input(EXAMPLE).unwrap();

        dbg!(all);

        assert_eq!(true, false);
    }

    const EXAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part_one() {
        let result = part_one(EXAMPLE);
        assert_eq!(result, None);
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(EXAMPLE);
    //     assert_eq!(result, None);
    // }
}
