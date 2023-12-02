use std::str::FromStr;

use regex::Regex;

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

pub fn part_two(input: &str) -> Option<u32> {
    let mut calibrations: Vec<u32> = vec![];
    let reg = Regex::new(r"(?<cstr>\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();

    for line in input.lines() {
        let cstrs: Vec<&str> = reg
            .captures_iter(line)
            .map(|c| {
                let cstr = c.name("cstr").unwrap().as_str();
                match cstr {
                    "one" => "1",
                    "two" => "2",
                    "three" => "3",
                    "four" => "4",
                    "five" => "5",
                    "six" => "6",
                    "seven" => "7",
                    "eight" => "8",
                    "nine" => "9",
                    _ => cstr,
                }
            })
            .collect();

        let (first, last) = (cstrs.first().unwrap(), cstrs.last().unwrap());
        let num_str = format!("{first}{last}");
        let num = u32::from_str(&num_str).unwrap();

        calibrations.push(num);
    }

    let mut total: u32 = 0;
    for num in calibrations {
        total += num;
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        assert_eq!(result, Some(281));
    }
}
