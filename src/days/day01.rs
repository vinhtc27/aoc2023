use crate::{DayResult, IntoDayResult};

pub fn part1(input: &'static str, _: bool) -> anyhow::Result<DayResult> {
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

pub fn part2(input: &'static str, _: bool) -> anyhow::Result<DayResult> {
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

#[cfg(test)]
mod tests {
    use super::part1;
    use super::part2;
    use crate::{Answers, DayResult};

    #[test]
    fn test_part1() {
        let example1 = part1(include_str!("../../input/day1/example1.txt"), false);
        assert_eq!(
            example1.unwrap(),
            DayResult {
                answers: Some(Answers::U32(142)),
            }
        );
        let real1 = part1(include_str!("../../input/day1/real1.txt"), false);
        assert_eq!(
            real1.unwrap(),
            DayResult {
                answers: Some(Answers::U32(54601)),
            }
        );
    }

    #[test]
    fn test_part2() {
        let example2 = part2(include_str!("../../input/day1/example2.txt"), false);
        assert_eq!(
            example2.unwrap(),
            DayResult {
                answers: Some(Answers::U32(281)),
            }
        );
        let real2 = part2(include_str!("../../input/day1/real2.txt"), false);
        assert_eq!(
            real2.unwrap(),
            DayResult {
                answers: Some(Answers::U32(54078)),
            }
        );
    }
}
