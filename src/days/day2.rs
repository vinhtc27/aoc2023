use crate::{DayResult, IntoDayResult};

pub fn part1_sol1(input: &'static str) -> anyhow::Result<DayResult> {
    let mut part1: u32 = 0;
    for (game, line) in input.lines().enumerate() {
        let sets = line.split(": ").last().unwrap().split("; ");
        let mut is_valid = true;
        'GAME: for set in sets {
            for number_color in set.split(", ") {
                let (number, color) = number_color.split_once(" ").unwrap();
                let number_u32 = number.parse::<u32>().unwrap();
                match color {
                    "red" => {
                        if number_u32 > 12 {
                            is_valid = false;
                            break 'GAME;
                        }
                    }
                    "green" => {
                        if number_u32 > 13 {
                            is_valid = false;
                            break 'GAME;
                        }
                    }
                    "blue" => {
                        if number_u32 > 14 {
                            is_valid = false;
                            break 'GAME;
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
        if is_valid {
            part1 += game as u32 + 1;
        }
    }
    (part1).into_result()
}

pub fn part2_sol1(input: &'static str) -> anyhow::Result<DayResult> {
    let mut part2: u32 = 0;
    for line in input.lines() {
        let sets = line.split(": ").last().unwrap().split("; ");
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for set in sets {
            for number_color in set.split(", ") {
                let (number, color) = number_color.split_once(" ").unwrap();
                let number_u32 = number.parse::<u32>().unwrap();
                match color {
                    "red" => {
                        if number_u32 > min_red {
                            min_red = number_u32;
                        }
                    }
                    "green" => {
                        if number_u32 > min_green {
                            min_green = number_u32;
                        }
                    }
                    "blue" => {
                        if number_u32 > min_blue {
                            min_blue = number_u32;
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
        part2 += min_red * min_green * min_blue;
    }
    (part2).into_result()
}

#[cfg(test)]
mod tests {
    use super::part1_sol1;
    use super::part2_sol1;
    use crate::{Answers, DayResult};

    #[test]
    fn test_part1() {
        let example1 = part1_sol1(include_str!("../../input/day2/example1.txt"));
        assert_eq!(
            example1.unwrap(),
            DayResult {
                answers: Some(Answers::U32(8)),
            }
        );
        let real1 = part1_sol1(include_str!("../../input/day2/real1.txt"));
        assert_eq!(
            real1.unwrap(),
            DayResult {
                answers: Some(Answers::U32(2683)),
            }
        );
    }

    #[test]
    fn test_part2() {
        let example2 = part2_sol1(include_str!("../../input/day2/example2.txt"));
        assert_eq!(
            example2.unwrap(),
            DayResult {
                answers: Some(Answers::U32(2286)),
            }
        );
        let real2 = part2_sol1(include_str!("../../input/day2/real2.txt"));
        assert_eq!(
            real2.unwrap(),
            DayResult {
                answers: Some(Answers::U32(0)),
            }
        );
    }
}
