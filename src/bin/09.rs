use parsers::parse_input;

advent_of_code::solution!(9);

mod parsers {
    use nom::{
        character::complete::{char, digit1, line_ending, space1},
        combinator::{map, opt},
        multi::separated_list1,
        sequence::pair,
        IResult,
    };

    pub fn parse_i32(input: &str) -> IResult<&str, i32> {
        map(pair(opt(char('-')), digit1), |(minus, digs)| {
            let num: i32 = (digs as &str).parse().unwrap();

            match minus {
                Some(_) => -num,
                None => num,
            }
        })(input)
    }

    pub fn parse_nums(input: &str) -> IResult<&str, Vec<i32>> {
        separated_list1(space1, parse_i32)(input)
    }

    pub fn parse_input(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
        separated_list1(line_ending, parse_nums)(input)
    }
}

fn diffs_of(vec: &[i32]) -> Vec<i32> {
    let mut diffs = vec![];

    for i in 0..vec.len() {
        let left = vec.get(i).unwrap();
        let right = vec.get(i + 1);

        if let Some(right) = right {
            diffs.push(right - left)
        }
    }

    diffs
}

fn next_num(vec: &[i32]) -> i32 {
    //
    if vec.iter().all(|x| x == &0) {
        return 0;
    }

    let diffs = diffs_of(vec);

    vec.last().unwrap() + next_num(&diffs)
}

#[allow(unused_variables)]
#[allow(unused_must_use)]
pub fn part_one(input: &str) -> Option<i32> {
    let (_, sequences) = parse_input(input).unwrap();

    Some(sequences.iter().map(|s| next_num(s)).sum())
}

#[allow(unused_variables)]
#[allow(unused_must_use)]
pub fn part_two(input: &str) -> Option<i32> {
    let (_, sequences) = parse_input(input).unwrap();

    Some(
        sequences
            .into_iter()
            .map(|s| s.into_iter().rev().collect::<Vec<i32>>())
            .map(|s| next_num(&s))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_part_one() {
        let result = part_one(EXAMPLE);
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_one_line_1() {
        let line1 = EXAMPLE
            .lines()
            .take(1)
            .collect::<Vec<&str>>()
            .first()
            .unwrap()
            .to_owned();

        let result = part_one(line1);

        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_i32_parser() {
        let (_, nums) = parsers::parse_nums("12 145 -987 -1 4 21 -82").unwrap();

        assert_eq!(nums, vec![12, 145, -987, -1, 4, 21, -82]);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(EXAMPLE);
        assert_eq!(result, Some(2));
    }
}
