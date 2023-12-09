use std::collections::HashMap;

use parsers::parse_input;

advent_of_code::solution!(8);

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Left,
    Right,
}

#[derive(PartialEq)]
pub struct Node<'a> {
    label: &'a str,
    left_node: &'a str,
    right_node: &'a str,
}

pub struct NodeMap<'a> {
    nodes: HashMap<&'a str, Node<'a>>,
    dirs: Vec<Direction>,
}

impl<'a> NodeMap<'a> {
    fn navigate(&self, node: &Node, dir: &Direction) -> &Node {
        let label_to_find = match dir {
            Direction::Left => node.left_node,
            Direction::Right => node.right_node,
        };

        let found = self.nodes.get(label_to_find);

        found.unwrap()
    }

    fn get_bounds(&self) -> (&Node, &Node) {
        (
            self.nodes.get("AAA").unwrap(),
            self.nodes.get("ZZZ").unwrap(),
        )
    }
}

mod parsers {
    use nom::bytes::complete::tag;
    use nom::character::complete::{alpha1, char, line_ending, space1};
    use nom::combinator::{eof, map};
    use nom::multi::{fold_many1, many0, many1};
    use nom::sequence::{delimited, terminated};
    use nom::IResult;
    use nom::{branch::alt, sequence::separated_pair};

    use super::*;

    pub fn dir(input: &str) -> IResult<&str, Direction> {
        alt((
            map(char('L'), |_| Direction::Left),
            map(char('R'), |_| Direction::Right),
        ))(input)
    }

    pub fn parse_directions(input: &str) -> IResult<&str, Vec<Direction>> {
        many1(dir)(input)
    }

    pub fn parse_node(input: &str) -> IResult<&str, Node> {
        map(
            separated_pair(
                alpha1,
                tag(" = "),
                delimited(
                    char('('),
                    separated_pair(alpha1, tag(", "), alpha1),
                    char(')'),
                ),
            ),
            |(label, (left_node, right_node))| {
                //
                Node {
                    label,
                    left_node,
                    right_node,
                }
            },
        )(input)
    }

    pub fn parse_input(input: &str) -> IResult<&str, (Vec<Direction>, HashMap<&str, Node>)> {
        separated_pair(
            parse_directions,
            many0(alt((line_ending, space1))),
            fold_many1(
                terminated(parse_node, alt((line_ending, eof))),
                HashMap::new,
                |mut map, node| {
                    map.insert(node.label, node);

                    map
                },
            ),
        )(input)
    }
}

fn create_map(input: &str) -> NodeMap {
    parse_input(input)
        .map(|(_, (dirs, nodes))| NodeMap { nodes, dirs })
        .unwrap()
}

#[allow(unused_variables)]
#[allow(unused_must_use)]
pub fn part_one(input: &str) -> Option<u32> {
    let map = create_map(input);
    let (start, end) = map.get_bounds();

    let length = map.dirs.len();

    let mut curr = start;
    let mut moves: u32 = 0;

    while curr != end {
        let turn = map.dirs.get(moves as usize % length).unwrap();

        curr = map.navigate(curr, turn);
        moves += 1;
    }

    Some(moves)
}

#[allow(unused_variables)]
#[allow(unused_must_use)]
pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const EXAMPLE_2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_part_one() {
        let result = part_one(EXAMPLE);
        assert_eq!(result, Some(2));

        let result2 = part_one(EXAMPLE_2);
        assert_eq!(result2, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(EXAMPLE);
        assert_eq!(result, None);
    }
}
