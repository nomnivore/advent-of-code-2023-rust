advent_of_code::solution!(2);

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

pub fn part_one(input: &str) -> Option<u32> {
    let mut possible_games = 0;
    input
        .lines()
        .map(|line| {
            let (prefix, game) = line.split_once(": ").unwrap();
            let game_id: u32 = prefix[5..].parse().unwrap();

            (game_id, game)
        })
        .for_each(|(game_id, game)| {
            let mut valid = true;
            game.split("; ").for_each(|round| {
                if !valid {
                    return;
                };
                round.split(", ").for_each(|hand| {
                    if !valid {
                        return;
                    }
                    let (num_str, color) = hand.split_once(' ').unwrap();
                    let num: u32 = num_str.parse().unwrap();

                    match color {
                        "red" => {
                            if num > MAX_RED {
                                valid = false
                            }
                        }
                        "green" => {
                            if num > MAX_GREEN {
                                valid = false
                            }
                        }
                        "blue" => {
                            if num > MAX_BLUE {
                                valid = false
                            }
                        }
                        _ => panic!("unexpected data"),
                    };
                });
            });

            if valid {
                possible_games += game_id
            }
        });

    Some(possible_games)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, Some(8));
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
