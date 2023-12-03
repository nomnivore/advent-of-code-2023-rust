use std::collections::HashMap;

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

pub fn part_two(_input: &str) -> Option<u32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(result, Some(4361));
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two();
    //     assert_eq!(result, None);
    // }
}
