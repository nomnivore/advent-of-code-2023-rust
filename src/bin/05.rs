use parsers::parse_input;

advent_of_code::solution!(5);

mod parsers {

    use nom::{
        branch::alt,
        bytes::complete::{tag, take_until},
        character::complete::{digit1, line_ending, newline, space0, space1},
        combinator::eof,
        multi::{many0, many1},
        sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
        IResult,
    };

    fn parse_usize(input: &str) -> IResult<&str, usize> {
        digit1(input).map(|(input, value)| (input, value.parse().unwrap()))
    }

    pub fn parse_header(input: &str) -> IResult<&str, Vec<usize>> {
        let numbers = many1(delimited(space0, parse_usize, space0));
        preceded(pair(tag("seeds:"), space1), numbers)(input)
    }

    pub type ChartTitle<'a> = (&'a str, &'a str);
    pub type Mapping = (usize, usize, usize);
    pub type SeedsToPlant = Vec<usize>;
    pub type Chart<'a> = (ChartTitle<'a>, Vec<Mapping>);
    pub fn parse_map(input: &str) -> IResult<&str, (ChartTitle, Vec<Mapping>)> {
        let from = take_until("-");
        let to = take_until(" ");
        let delimited_usize = |i| delimited(space0, parse_usize, space0)(i);
        pair(
            terminated(
                separated_pair(from, tag("-to-"), to),
                pair(tag(" map:"), line_ending),
            ),
            many1(terminated(
                tuple((delimited_usize, delimited_usize, delimited_usize)),
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

struct Chart {
    mappings: Vec<ChartMapping>,
}

impl Chart {
    fn convert(&self, num: usize) -> usize {
        self.mappings
            .iter()
            .find_map(|x| x.convert(num))
            .unwrap_or(num)
    }
}

struct ChartMapping {
    source: usize,
    dest: usize,
    range: usize,
}

impl ChartMapping {
    fn in_range(&self, num: usize) -> bool {
        self.source <= num && num < self.source + self.range
    }

    fn convert(&self, num: usize) -> Option<usize> {
        self.in_range(num).then(|| {
            // get difference
            let diff = num - self.source;

            self.dest + diff
        })
    }
}

impl From<(usize, usize, usize)> for ChartMapping {
    fn from(tuple: (usize, usize, usize)) -> Self {
        Self {
            dest: tuple.0,
            source: tuple.1,
            range: tuple.2,
        }
    }
}

fn prepare(input: &str) -> (Vec<usize>, Vec<Chart>) {
    let (_, (seeds, charts)) = parse_input(input).unwrap();

    let map = charts
        .into_iter()
        .map(|(key, chart)| Chart {
            mappings: chart.into_iter().map(ChartMapping::from).collect(),
        })
        .collect();

    (seeds, map)
}

#[allow(unused_variables)]
#[allow(unused_must_use)]
pub fn part_one(input: &str) -> Option<usize> {
    let (seeds, maps) = prepare(input);

    let x = seeds
        .into_iter()
        .map(|seed| maps.iter().fold(seed, |n, chart| chart.convert(n)))
        .min();

    x
}

#[allow(unused_variables)]
#[allow(unused_must_use)]
pub fn part_two(input: &str) -> Option<usize> {
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
            "abc-to-xyzhjkl map:
1 2 3
4 5 6
",
        )
        .unwrap();

        assert_eq!(input, "");
        assert_eq!(from, "abc");
        assert_eq!(to, "xyzhjkl");
        assert_eq!(mappings, [(1, 2, 3), (4, 5, 6)]);
    }

    #[test]
    fn test_parse_input() {
        let (input, (seeds, maps)) = parse_input(EXAMPLE).unwrap();

        // not exhaustive testing but should be enough to check what we need to
        assert_eq!(input, "");
        assert_eq!(seeds, [79, 14, 55, 13]);
        assert_eq!(maps.len(), 7);
        assert_eq!(maps.first().unwrap().0, ("seed", "soil"));
        assert_eq!(maps.last().unwrap().0, ("humidity", "location"));
        assert_eq!(
            maps.last().unwrap().1.first().unwrap(),
            &(60usize, 56usize, 37usize)
        );
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
        assert_eq!(result, Some(35));
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(EXAMPLE);
    //     assert_eq!(result, None);
    // }
}
