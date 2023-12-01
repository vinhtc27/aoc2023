use crate::{DayResult, IntoDayResult};

pub fn run(input: &'static str, _: bool) -> anyhow::Result<DayResult> {
    let part1: u32 = input
        .lines()
        .map(|line| {
            println!("{}", line);
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

    let nums: Vec<&str> = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut part2 = 0;
    for line in input.lines() {
        let mut first = None;
        let mut last = None;

        for (i, c) in line.chars().enumerate() {
            let mut cur = None;

            if c.is_digit(10) {
                cur = Some(c.to_digit(10).unwrap() as u32);
            }
            for (j, num) in nums.iter().enumerate() {
                if line.len() >= num.len() {
                    if i <= line.len() - num.len() {
                        if line[i..(i + num.len())] == **num {
                            cur = Some(j as u32 + 1);
                            break;
                        }
                    }
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

    (part1, part2).into_result()
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::{Answers, DayResult};

    #[test]
    fn test_example_answers() {
        let result = run(include_str!("../../input/test/01.txt"), false);
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(Answers::U32(351)),
                part2: Some(Answers::U32(340)),
            }
        );
    }

    #[test]
    fn test_answers() {
        let result = run(include_str!("../../input/real/01.txt"), false);
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(Answers::U32(54601)),
                part2: Some(Answers::U32(54078)),
            }
        );
    }
}
