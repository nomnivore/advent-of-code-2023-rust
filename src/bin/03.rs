use std::collections::{HashMap, HashSet};

advent_of_code::solution!(3);

fn is_symbol(x: &char) -> bool {
    x != &'.' && x.is_ascii_punctuation()
}

const DIRECTIONS: [(i32, i32); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (1, 1),
    (1, -1),
    (-1, 0),
    (-1, 1),
    (-1, -1),
];

fn near_symbol(symmap: &HashMap<(i32, i32), bool>, row: i32, col: i32) -> bool {
    DIRECTIONS
        .iter()
        .any(|(row_dir, col_dir)| symmap.get(&(row + row_dir, col + col_dir)).is_some())
}

pub fn part_one(input: &str) -> Option<u32> {
    // iterate over each character
    // if it is a number(digit), add it to current_number and check all directions for a symbol
    // if a symbol is found, the whole number is counted towards the sum
    // if it is a non-number, clear (and conditionally add) the current_number and continue

    // first pass to collect all symbols and store as 'known symbols'
    let mut symmap: HashMap<(i32, i32), bool> = HashMap::new();
    let mut curr_line = 0;
    input.lines().for_each(|line| {
        line.char_indices().for_each(|(i, c)| {
            if is_symbol(&c) {
                symmap.insert((curr_line, i as i32), true);
            }
        });

        curr_line += 1
    });

    let mut sum: u32 = 0;

    curr_line = 0;
    input.lines().for_each(|line| {
        let mut curr_number = String::new();
        let mut is_valid = false;

        line.char_indices().for_each(|(i, c)| {
            if c.is_ascii_digit() {
                curr_number.push(c);

                // does rust short-circuit expressions like this?
                is_valid = is_valid || near_symbol(&symmap, curr_line, i as i32)
            } else {
                if is_valid {
                    let num: u32 = curr_number.parse().unwrap();

                    sum += num
                }

                curr_number = String::new();
                is_valid = false
            }
        });

        if is_valid {
            let num: u32 = curr_number.parse().unwrap();

            sum += num
        }

        curr_line += 1;
    });

    Some(sum)
}

type Coord = (i32, i32);
type GearRefs = Vec<u32>;
type GearMap = HashMap<Coord, GearRefs>;

fn near_gears(symmap: &GearMap, row: i32, col: i32) -> Vec<Coord> {
    let mut nearby_gears = vec![];
    for (row_dir, col_dir) in DIRECTIONS.iter() {
        let coord: Coord = (row + row_dir, col + col_dir);
        if symmap.get(&coord).is_some() {
            nearby_gears.push(coord);
        }
    }

    nearby_gears
}

pub fn part_two(input: &str) -> Option<u32> {
    // iterate over each character
    // if it is a number(digit), add it to current_number and check all directions for a symbol
    // if a symbol is found, the whole number is counted towards the sum
    // if it is a non-number, clear (and conditionally add) the current_number and continue

    // first pass to collect all symbols and store as 'known symbols'
    let mut symmap: GearMap = HashMap::new();
    input.lines().enumerate().for_each(|(row, line)| {
        line.char_indices().for_each(|(col, c)| {
            if is_symbol(&c) {
                symmap.insert((row as i32, col as i32), vec![]);
            }
        });
    });

    input.lines().enumerate().for_each(|(row, line)| {
        let mut curr_number = String::new();
        let mut nearby_gears: HashSet<Coord> = HashSet::new();

        line.char_indices().for_each(|(col, c)| {
            if c.is_ascii_digit() {
                curr_number.push(c);

                // check for gears
                near_gears(&symmap, row as i32, col as i32)
                    .iter()
                    .for_each(|coord| {
                        nearby_gears.insert(*coord);
                    });
            } else if !curr_number.is_empty() {
                let num: u32 = curr_number.parse().unwrap();

                nearby_gears.iter().for_each(|gear| {
                    let x = symmap.get_mut(gear).unwrap();
                    x.push(num);
                });

                curr_number = String::new();
                nearby_gears.clear();
            }
        });

        if !curr_number.is_empty() {
            let num: u32 = curr_number.parse().unwrap();

            nearby_gears.iter().for_each(|gear| {
                let x = symmap.get_mut(gear).unwrap();
                x.push(num);
            });
        }
    });

    let mut sum: u32 = 0;

    symmap.iter().for_each(|(_, nums)| {
        if nums.len() == 2 {
            sum += nums[0] * nums[1];
        }
    });

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    #[test]
    fn test_part_one() {
        let result = part_one(EXAMPLE);
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(EXAMPLE);
        assert_eq!(result, Some(467835));
    }
}
