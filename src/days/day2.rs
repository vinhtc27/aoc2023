use crate::{Answers, IntoAnswers};

pub fn part1_sol1(input: &str) -> anyhow::Result<Answers> {
    let mut part1: u32 = 0;
    for (game, line) in input.lines().enumerate() {
        let sets = line.split(": ").last().unwrap().split("; ");
        let mut is_valid = true;
        'GAME: for set in sets {
            for number_color in set.split(", ") {
                let (number, color) = number_color.split_once(' ').unwrap();
                let amount = number.parse::<u32>().unwrap();
                match color {
                    "red" => {
                        if amount > 12 {
                            is_valid = false;
                            break 'GAME;
                        }
                    }
                    "green" => {
                        if amount > 13 {
                            is_valid = false;
                            break 'GAME;
                        }
                    }
                    "blue" => {
                        if amount > 14 {
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
    (part1).into_answer()
}

pub fn part1_sol2(input: &str) -> anyhow::Result<Answers> {
    let mut part1 = 0;
    for (game, line) in input.lines().enumerate() {
        let mut set_start_index = line.find(':').unwrap() + 2;
        let line = line.as_bytes();
        let game = game as u32 + 1;
        let mut is_valid = true;
        while set_start_index < line.len() {
            match line[set_start_index..set_start_index + 2] {
                [i, b' '] => {
                    let amount = i - 48;
                    match line[set_start_index + 2] {
                        b'r' => {
                            if amount > 12 {
                                is_valid = false;
                                break;
                            }
                            set_start_index += 7;
                        }
                        b'g' => {
                            if amount > 13 {
                                is_valid = false;
                                break;
                            }
                            set_start_index += 9;
                        }
                        b'b' => {
                            if amount > 14 {
                                is_valid = false;
                                break;
                            }
                            set_start_index += 8;
                        }
                        _ => unreachable!(),
                    };
                }
                [i, j] => {
                    let amount = (i - 48) * 10 + j - 48;
                    match line[set_start_index + 3] {
                        b'r' => {
                            if amount > 12 {
                                is_valid = false;
                                break;
                            }
                            set_start_index += 8;
                        }
                        b'g' => {
                            if amount > 13 {
                                is_valid = false;
                                break;
                            }
                            set_start_index += 10;
                        }
                        b'b' => {
                            if amount > 14 {
                                is_valid = false;
                                break;
                            }
                            set_start_index += 9;
                        }
                        _ => unreachable!(),
                    };
                }
                _ => unreachable!(),
            };
        }
        if is_valid {
            part1 += game;
        }
    }
    (part1).into_answer()
}

pub fn part2_sol1(input: &str) -> anyhow::Result<Answers> {
    let mut part2: u32 = 0;
    for line in input.lines() {
        let sets = line.split(": ").last().unwrap().split("; ");
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for set in sets {
            for number_color in set.split(", ") {
                let (number, color) = number_color.split_once(' ').unwrap();
                let amount = number.parse::<u32>().unwrap();
                match color {
                    "red" => {
                        if amount > min_red {
                            min_red = amount;
                        }
                    }
                    "green" => {
                        if amount > min_green {
                            min_green = amount;
                        }
                    }
                    "blue" => {
                        if amount > min_blue {
                            min_blue = amount;
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
        part2 += min_red * min_green * min_blue;
    }
    (part2).into_answer()
}

pub fn part2_sol2(_input: &str) -> anyhow::Result<Answers> {
    let part2 = 0;
    (part2).into_answer()
}

#[cfg(test)]
mod tests {
    use super::part1_sol1;
    use super::part2_sol1;
    use crate::Answers;

    #[test]
    fn test_part1() {
        let example1 = part1_sol1(include_str!("../../input/day2/example1.txt"));
        assert_eq!(example1.unwrap(), Answers::U32(8));
        let real1 = part1_sol1(include_str!("../../input/day2/real1.txt"));
        assert_eq!(real1.unwrap(), Answers::U32(2683));
    }

    #[test]
    fn test_part2() {
        let example2 = part2_sol1(include_str!("../../input/day2/example2.txt"));
        assert_eq!(example2.unwrap(), Answers::U32(2286));
        let real2 = part2_sol1(include_str!("../../input/day2/real2.txt"));
        assert_eq!(real2.unwrap(), Answers::U32(49710));
    }
}
