use std::collections::HashMap;

use crate::{Answers, IntoAnswers};

pub fn part1_sol1(input: &str) -> anyhow::Result<Answers> {
    let mut part1: u32 = 0;
    let maps: Vec<Vec<char>> = input
        .lines()
        .map(|line| {
            let mut char_line: Vec<char> = line.chars().collect();
            char_line.push('.');
            char_line
        })
        .collect();

    for (i, line) in maps.iter().enumerate() {
        let mut current_num = String::with_capacity(3);
        let mut j_start_some = None;
        for (j, c) in line.iter().enumerate() {
            if c.is_ascii_digit() {
                current_num.push(*c);
                if j_start_some.is_none() {
                    j_start_some = Some(j);
                }
            } else {
                if j_start_some.is_some() {
                    let number = current_num.parse::<u32>().unwrap();
                    let j_start = j_start_some.unwrap();
                    let j_end = j - 1;
                    let mut is_valid = false;
                    if j_start > 0 {
                        // Check left
                        if &maps[i][j_start - 1] != &'.' {
                            is_valid = true;
                        }
                    }
                    if j_end + 1 < line.len() {
                        // Check right
                        if &maps[i][j_end + 1] != &'.' {
                            is_valid = true;
                        }
                    }
                    if i > 0 && j_start > 0 {
                        // Check up left
                        if &maps[i - 1][j_start - 1] != &'.' {
                            is_valid = true;
                        }
                    }
                    if i + 1 < maps.len() && j_start > 0 {
                        // Check down left
                        if &maps[i + 1][j_start - 1] != &'.' {
                            is_valid = true;
                        }
                    }
                    if i > 0 && j_end + 1 < line.len() {
                        // Check up right
                        if &maps[i - 1][j_end + 1] != &'.' {
                            is_valid = true;
                        }
                    }
                    if i + 1 < maps.len() && j_end + 1 < line.len() {
                        // Check down right
                        if &maps[i + 1][j_end + 1] != &'.' {
                            is_valid = true;
                        }
                    }
                    if i > 0 {
                        // Check up between
                        for k in j_start..j {
                            if &maps[i - 1][k] != &'.' {
                                is_valid = true;
                            }
                        }
                    }
                    if i + 1 < maps.len() {
                        // Check down between
                        for k in j_start..j {
                            if &maps[i + 1][k] != &'.' {
                                is_valid = true;
                            }
                        }
                    }
                    if is_valid {
                        part1 += number;
                    }
                    current_num.clear();
                    j_start_some = None;
                }
            }
        }
    }
    (part1).into_answer()
}

pub fn part2_sol1(input: &str) -> anyhow::Result<Answers> {
    let mut part2: u32 = 0;
    let maps: Vec<Vec<char>> = input
        .lines()
        .map(|line| {
            let mut char_line: Vec<char> = line.chars().collect();
            char_line.push('.');
            char_line
        })
        .collect();

    let mut gears: HashMap<(u32, u32), Vec<u32>> =
        HashMap::with_capacity(input.matches('*').count());
    for (i, line) in maps.iter().enumerate() {
        let mut current_num = String::with_capacity(3);
        let mut j_start_some = None;
        for (j, c) in line.iter().enumerate() {
            if c.is_ascii_digit() {
                current_num.push(*c);
                if j_start_some.is_none() {
                    j_start_some = Some(j);
                }
            } else {
                if j_start_some.is_some() {
                    let number = current_num.parse::<u32>().unwrap();
                    let j_start = j_start_some.unwrap();
                    let j_end = j - 1;
                    if j_start > 0 {
                        // Check left
                        if &maps[i][j_start - 1] == &'*' {
                            set_gears(&mut gears, (i as u32, j_start as u32 - 1), number);
                        }
                    }
                    if j_end + 1 < line.len() {
                        // Check right
                        if &maps[i][j_end + 1] == &'*' {
                            set_gears(&mut gears, (i as u32, j_end as u32 + 1), number);
                        }
                    }
                    if i > 0 && j_start > 0 {
                        // Check up left
                        if &maps[i - 1][j_start - 1] == &'*' {
                            set_gears(&mut gears, (i as u32 - 1, j_start as u32 - 1), number);
                        }
                    }
                    if i + 1 < maps.len() && j_start > 0 {
                        // Check down left
                        if &maps[i + 1][j_start - 1] == &'*' {
                            set_gears(&mut gears, (i as u32 + 1, j_start as u32 - 1), number);
                        }
                    }
                    if i > 0 && j_end + 1 < line.len() {
                        // Check up right
                        if &maps[i - 1][j_end + 1] == &'*' {
                            set_gears(&mut gears, (i as u32 - 1, j_end as u32 + 1), number);
                        }
                    }
                    if i + 1 < maps.len() && j_end + 1 < line.len() {
                        // Check down right
                        if &maps[i + 1][j_end + 1] == &'*' {
                            set_gears(&mut gears, (i as u32 + 1, j_end as u32 + 1), number);
                        }
                    }
                    if i > 0 {
                        // Check up between
                        for k in j_start..j {
                            if &maps[i - 1][k] == &'*' {
                                set_gears(&mut gears, (i as u32 - 1, k as u32), number);
                            }
                        }
                    }
                    if i + 1 < maps.len() {
                        // Check down between
                        for k in j_start..j {
                            if &maps[i + 1][k] == &'*' {
                                set_gears(&mut gears, (i as u32 + 1, k as u32), number);
                            }
                        }
                    }
                    current_num.clear();
                    j_start_some = None;
                }
            }
        }
    }
    for (_, numbers) in gears.iter() {
        if numbers.len() == 2 {
            part2 += numbers[0] * numbers[1];
        }
    }
    (part2).into_answer()
}

#[inline]
fn set_gears(gears: &mut HashMap<(u32, u32), Vec<u32>>, key: (u32, u32), number: u32) {
    if gears.get(&key).is_none() {
        gears.insert(key, vec![number]);
    } else {
        gears.get_mut(&key).unwrap().push(number);
    }
}

#[cfg(test)]
mod tests {
    use super::part1_sol1;
    use super::part2_sol1;
    use crate::Answers;

    #[test]
    fn test_part1() {
        let example1 = part1_sol1(include_str!("../../input/day3/example1.txt"));
        assert_eq!(example1.unwrap(), Answers::U32(4361));
        let real1 = part1_sol1(include_str!("../../input/day3/real1.txt"));
        assert_eq!(real1.unwrap(), Answers::U32(535351));
    }

    #[test]
    fn test_part2() {
        let example2 = part2_sol1(include_str!("../../input/day3/example2.txt"));
        assert_eq!(example2.unwrap(), Answers::U32(467835));
        let real2 = part2_sol1(include_str!("../../input/day3/real2.txt"));
        assert_eq!(real2.unwrap(), Answers::U32(87287096));
    }
}
