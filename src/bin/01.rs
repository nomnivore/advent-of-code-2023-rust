use std::str::FromStr;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut calibrations: Vec<u32> = vec![];
    for line in input.lines() {
        let mut first_number = None;
        let mut last_number = None;

        // find the first number
        for ch in line.chars() {
            if ch.is_ascii_digit() {
                first_number = Some(ch.to_string());
                break;
            }
        }

        // find the last number
        for ch in line.chars().rev() {
            if ch.is_ascii_digit() {
                last_number = Some(ch.to_string());
                break;
            }
        }

        if let (Some(first_number), Some(last_number)) = (first_number, last_number) {
            let number = {
                let joined = format!("{}{}", first_number, last_number);
                u32::from_str(joined.as_str())
            };
            match number {
                Ok(number) => calibrations.push(number),
                Err(_) => panic!("failed to parse"),
            }
        }
    }

    let mut total = 0;

    for val in calibrations {
        total += val
    }

    Some(total)
}

pub fn part_two(_input: &str) -> Option<u32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
