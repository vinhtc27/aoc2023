use crate::{Answers, IntoAnswers};

pub fn part1_sol1(_input: &str) -> anyhow::Result<Answers> {
    let part1: u32 = 0;
    (part1).into_answer()
}

pub fn part2_sol1(_input: &str) -> anyhow::Result<Answers> {
    let part2: u32 = 0;
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
        assert_eq!(example1.unwrap(), Answers::U32(0));
        let real1 = part1_sol1(include_str!("../../input/day2/real1.txt"));
        assert_eq!(real1.unwrap(), Answers::U32(0));
    }

    #[test]
    fn test_part2() {
        let example2 = part2_sol1(include_str!("../../input/day2/example2.txt"));
        assert_eq!(example2.unwrap(), Answers::U32(0));
        let real2 = part2_sol1(include_str!("../../input/day2/real2.txt"));
        assert_eq!(real2.unwrap(), Answers::U32(0));
    }
}
