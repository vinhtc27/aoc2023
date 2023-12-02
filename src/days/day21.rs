use crate::{DayResult, IntoDayResult};

pub fn part1_sol1(_input: &'static str) -> anyhow::Result<DayResult> {
    let part1: u32 = 0;
    (part1).into_result()
}

pub fn part2_sol1(_input: &'static str) -> anyhow::Result<DayResult> {
    let part2: u32 = 0;
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
                answers: Some(Answers::U32(0)),
            }
        );
        let real1 = part1_sol1(include_str!("../../input/day2/real1.txt"));
        assert_eq!(
            real1.unwrap(),
            DayResult {
                answers: Some(Answers::U32(0)),
            }
        );
    }

    #[test]
    fn test_part2() {
        let example2 = part2_sol1(include_str!("../../input/day2/example2.txt"));
        assert_eq!(
            example2.unwrap(),
            DayResult {
                answers: Some(Answers::U32(0)),
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
