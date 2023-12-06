use crate::{Answers, IntoAnswers};

pub fn part1_sol1(input: &str) -> anyhow::Result<Answers> {
    let times: Vec<u32> = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    let distances: Vec<u32> = input
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    let mut part1: u32 = 1;
    for (time, distance) in times.iter().zip(distances.iter()) {
        let mut ways = 0;
        for i in 1..*time {
            if (time - i) * i >= *distance {
                ways += 1;
            }
        }
        part1 *= ways
    }
    (part1).into_answer()
}

pub fn part1_sol2(input: &str) -> anyhow::Result<Answers> {
    let mut part1: u32 = 1;
    for (time, distance) in input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .zip(
            input
                .lines()
                .last()
                .unwrap()
                .split_whitespace()
                .filter_map(|s| s.parse().ok()),
        )
        .collect::<Vec<(u32, u32)>>()
    {
        let mut low_speed = 0;
        let mut high_speed = time - 1;
        while (time - low_speed) * low_speed < distance {
            low_speed += 1;
        }
        while (time - high_speed) * high_speed < distance {
            high_speed -= 1;
        }
        part1 *= high_speed - low_speed + 1
    }
    (part1).into_answer()
}

pub fn part2_sol1(input: &str) -> anyhow::Result<Answers> {
    let times = input.lines().next().unwrap()[5..]
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();
    let distances = input.lines().last().unwrap()[9..]
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();
    let mut part2: u64 = 0;
    for i in 1..times {
        if (times - i) * i >= distances {
            part2 += 1;
        }
    }
    (part2).into_answer()
}

pub fn part2_sol2(input: &str) -> anyhow::Result<Answers> {
    let times = input.lines().next().unwrap()[5..]
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();
    let distances = input.lines().last().unwrap()[9..]
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();
    let mut low_speed = 0;
    let mut high_speed = times - 1;
    while (times - low_speed) * low_speed < distances {
        low_speed += 1;
    }
    while (times - high_speed) * high_speed < distances {
        high_speed -= 1;
    }
    let part2: u64 = high_speed - low_speed + 1;
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
        assert_eq!(example1.unwrap(), Answers::U32(288));
        let real1 = part1_sol1(include_str!("../../input/day2/real1.txt"));
        assert_eq!(real1.unwrap(), Answers::U32(1624896));
    }

    #[test]
    fn test_part2() {
        let example2 = part2_sol1(include_str!("../../input/day2/example2.txt"));
        assert_eq!(example2.unwrap(), Answers::U32(71503));
        let real2 = part2_sol1(include_str!("../../input/day2/real2.txt"));
        assert_eq!(real2.unwrap(), Answers::U32(32583852));
    }
}
