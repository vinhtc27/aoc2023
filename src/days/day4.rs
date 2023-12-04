use std::collections::HashSet;

use crate::{Answers, IntoAnswers};

pub fn part1_sol1(input: &str) -> anyhow::Result<Answers> {
    let part1: u32 = input
        .lines()
        .map(|line| {
            let mut splits = line.split(": ").last().unwrap().split(" | ");
            let wins: HashSet<&str> = splits
                .next()
                .unwrap()
                .split(" ")
                .filter(|s| s.trim().parse::<u32>().ok().is_some())
                .map(|s| s)
                .collect();
            let mut line_sum = 0;
            for num in splits.last().unwrap().split(" ") {
                if wins.contains(num.trim()) {
                    if line_sum == 0 {
                        line_sum = 1;
                    } else {
                        line_sum *= 2;
                    }
                }
            }
            line_sum
        })
        .sum();
    (part1).into_answer()
}

pub fn part2_sol1(input: &str) -> anyhow::Result<Answers> {
    let line = input.lines();
    let mut copy: Vec<usize> = vec![1; line.clone().count()];
    for (card, line) in line.enumerate() {
        let mut splits = line.split(": ").last().unwrap().split(" | ");
        let wins: HashSet<&str> = splits
            .next()
            .unwrap()
            .split(" ")
            .filter(|s| s.trim().parse::<u32>().ok().is_some())
            .map(|s| s)
            .collect();
        let card_copy = copy[card];
        let mut match_index = 0;
        for num in splits.last().unwrap().split(" ") {
            if wins.contains(num.trim()) {
                match_index += 1;
                copy[card + match_index] += card_copy;
            }
        }
    }
    let part2 = copy.iter().sum::<usize>();
    (part2).into_answer()
}

#[cfg(test)]
mod tests {
    use super::part1_sol1;
    use super::part2_sol1;
    use crate::Answers;

    #[test]
    fn test_part1() {
        let example1 = part1_sol1(include_str!("../../input/day4/example1.txt"));
        assert_eq!(example1.unwrap(), Answers::U32(13));
        let real1 = part1_sol1(include_str!("../../input/day4/real1.txt"));
        assert_eq!(real1.unwrap(), Answers::U32(21485));
    }

    #[test]
    fn test_part2() {
        let example2 = part2_sol1(include_str!("../../input/day4/example2.txt"));
        assert_eq!(example2.unwrap(), Answers::U32(0));
        let real2 = part2_sol1(include_str!("../../input/day4/real2.txt"));
        assert_eq!(real2.unwrap(), Answers::U32(0));
    }
}
