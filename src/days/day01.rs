use crate::{DayResult, IntoDayResult};

pub fn run(input: &'static str, _: bool) -> anyhow::Result<DayResult> {
    let part1: u32 = input
        .lines()
        .map(|line| {
            let first = line.chars().find(|c| c.is_numeric()).unwrap();
            let last = line.chars().rev().find(|c| c.is_numeric()).unwrap();
            format!("{}{}", first, last).parse::<u32>().unwrap()
        })
        .sum();

    let part2 = 0;

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
                part1: Some(Answers::U32(142)),
                part2: Some(Answers::U32(0)),
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
                part2: Some(Answers::U32(207_968)),
            }
        );
    }
}
