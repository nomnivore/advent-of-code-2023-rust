use num::Integer;
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
    use nom::character::complete::{alphanumeric1, char, line_ending, space1};
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
                alphanumeric1,
                tag(" = "),
                delimited(
                    char('('),
                    separated_pair(alphanumeric1, tag(", "), alphanumeric1),
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

fn lcm_of(nums: Vec<u64>) -> u64 {
    nums.into_iter().fold(1, |acc, num| acc.lcm(&num))
}

#[allow(unused_variables)]
#[allow(unused_must_use)]
pub fn part_two(input: &str) -> Option<u64> {
    let map = create_map(input);

    let length = map.dirs.len();

    let paths: Vec<&Node> = map
        .nodes
        .iter()
        .filter(|(label, node)| label.ends_with('A'))
        .map(|(label, node)| node)
        .collect();

    let dists = paths
        .into_iter()
        .map(|path| {
            let mut moves: u64 = 0;

            let mut curr = path;

            while !curr.label.ends_with('Z') {
                let turn = map.dirs.get(moves as usize % length).unwrap();
                curr = map.navigate(curr, turn);
                moves += 1
            }

            moves
        })
        .collect::<Vec<u64>>();

    // let mut dists = vec![];
    //
    // let mut stop = false;
    // let num_paths = paths.len();
    //
    // while !stop {
    //     let mut moves = 1;
    //
    //     for node in paths.iter_mut() {
    //         let turn = map.dirs.get(moves as usize % length).unwrap();
    //         let next = map.navigate(node, turn);
    //         if next.label.ends_with('Z') {
    //             dists.push(moves);
    //         }
    //
    //         *node = next;
    //
    //         moves += 1;
    //
    //         // break out of while loop if all paths have reached Z
    //         if dists.len() == num_paths {
    //             stop = true;
    //             break;
    //         }
    //     }
    // }

    Some(lcm_of(dbg!(dists)))
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
        let result = part_two(
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
        );

        assert_eq!(result, Some(6));
    }
}
