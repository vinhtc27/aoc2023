use crate::{DayResult, IntoDayResult};

pub fn part1_sol1(input: &'static str) -> anyhow::Result<DayResult> {
    let part1: u32 = input
        .lines()
        .map(|line| {
            let first = line.chars().find(|c| c.is_numeric()).unwrap();
            let last = line.chars().rev().find(|c| c.is_numeric()).unwrap();
            format!("{}{}", first, last).parse::<u32>().unwrap()
        })
        .sum();
    (part1).into_result()
}

pub fn part1_sol2(input: &'static str) -> anyhow::Result<DayResult> {
    let part1: u32 = input
        .lines()
        .map(|line| {
            let digits: Vec<u32> = line
                .chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| c.to_digit(10).unwrap())
                .collect();
            if digits.is_empty() {
                0
            } else {
                digits[0] * 10 + digits.last().unwrap_or(&0)
            }
        })
        .sum();
    (part1).into_result()
}

pub fn part1_sol3(input: &'static str) -> anyhow::Result<DayResult> {
    let part1: u32 = input
        .lines()
        .map(|line| {
            let mut first = None;
            let mut last = None;

            for c in line.chars() {
                if c.is_ascii_digit() {
                    if first.is_none() {
                        first = c.to_digit(10);
                    }
                    last = c.to_digit(10);
                }
            }

            first.unwrap_or(0) * 10 + last.unwrap_or(0)
        })
        .sum();
    (part1).into_result()
}

pub fn part1_sol4(input: &'static str) -> anyhow::Result<DayResult> {
    let part1: u32 = input
        .lines()
        .map(|line| {
            line.chars()
                .find(|c| c.is_numeric())
                .unwrap()
                .to_digit(10)
                .unwrap()
                * 10
                + line
                    .chars()
                    .rev()
                    .find(|c| c.is_numeric())
                    .unwrap()
                    .to_digit(10)
                    .unwrap()
        })
        .sum();

    (part1).into_result()
}

pub fn part1_sol5(input: &'static str) -> anyhow::Result<DayResult> {
    let mut numbers = Vec::with_capacity(20000);
    for line in input.lines() {
        let line = line.as_bytes();
        let mut first = 0u32;
        let mut last = 0u32;
        let mut initiated = false;
        for byte in line {
            if (48..58).contains(byte) {
                if initiated {
                    last = *byte as u32 - 48;
                } else {
                    first = *byte as u32 - 48;
                    last = first;
                    initiated = true;
                }
            }
        }
        let number = first * 10 + last;
        numbers.push(number);
    }
    let mut part1: u32 = 0;
    for number in numbers {
        part1 += number;
    }
    (part1).into_result()
}

pub fn part2_sol1(input: &'static str) -> anyhow::Result<DayResult> {
    let nums: Vec<&str> = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut part2 = 0;
    for line in input.lines() {
        let mut first = None;
        let mut last = None;

        for (i, c) in line.chars().enumerate() {
            let mut cur = None;

            if c.is_ascii_digit() {
                cur = c.to_digit(10);
            }
            for (j, num) in nums.iter().enumerate() {
                if line.len() >= num.len()
                    && i <= line.len() - num.len()
                    && line[i..(i + num.len())] == **num
                {
                    cur = Some(j as u32 + 1);
                    break;
                }
            }

            if cur.is_some() {
                if first.is_none() {
                    first = cur;
                }
                last = cur;
            }
        }

        part2 += first.unwrap() * 10 + last.unwrap()
    }

    (part2).into_result()
}

pub fn part2_sol2(input: &'static str) -> anyhow::Result<DayResult> {
    let part2: u32 = input
        .lines()
        .map(|line| {
            let mut line = (0..line.len()).filter_map(|index| {
                let reduced_line = &line[index..];
                let sub_line = if reduced_line.starts_with("one") {
                    '1'
                } else if reduced_line.starts_with("two") {
                    '2'
                } else if reduced_line.starts_with("three") {
                    '3'
                } else if reduced_line.starts_with("four") {
                    '4'
                } else if reduced_line.starts_with("five") {
                    '5'
                } else if reduced_line.starts_with("six") {
                    '6'
                } else if reduced_line.starts_with("seven") {
                    '7'
                } else if reduced_line.starts_with("eight") {
                    '8'
                } else if reduced_line.starts_with("nine") {
                    '9'
                } else {
                    reduced_line.chars().next().unwrap()
                };
                sub_line.to_digit(10)
            });
            let first = line.next().unwrap();
            match line.last() {
                Some(last) => first * 10 + last,
                None => first * 10 + first,
            }
        })
        .sum();
    (part2).into_result()
}

#[cfg(test)]
mod tests {
    use super::part1_sol4;
    use super::part2_sol2;
    use crate::{Answers, DayResult};

    #[test]
    fn test_part1() {
        let example1 = part1_sol4(include_str!("../../input/day1/example1.txt"));
        assert_eq!(
            example1.unwrap(),
            DayResult {
                answers: Some(Answers::U32(142)),
            }
        );
        let real1 = part1_sol4(include_str!("../../input/day1/real1.txt"));
        assert_eq!(
            real1.unwrap(),
            DayResult {
                answers: Some(Answers::U32(54601)),
            }
        );
    }

    #[test]
    fn test_part2() {
        let example2 = part2_sol2(include_str!("../../input/day1/example2.txt"));
        assert_eq!(
            example2.unwrap(),
            DayResult {
                answers: Some(Answers::U32(281)),
            }
        );
        let real2 = part2_sol2(include_str!("../../input/day1/real2.txt"));
        assert_eq!(
            real2.unwrap(),
            DayResult {
                answers: Some(Answers::U32(54078)),
            }
        );
    }
}
