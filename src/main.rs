use aoc2023::run_day_challenge;
use aoc2023::{days, DayChallenge};

fn main() -> anyhow::Result<()> {
    let is_test = std::env::var_os("TEST").is_some();
    let _ = get_days()
        .into_iter()
        .enumerate()
        .try_for_each(|(day, day_challenge)| run_day_challenge(day + 1, &day_challenge, is_test))?;

    Ok(())
}

fn get_days() -> Vec<DayChallenge> {
    use days::{day1, day2, day3};
    vec![
        DayChallenge {
            part1s: vec![
                day1::part1_sol1,
                day1::part1_sol2,
                day1::part1_sol3,
                day1::part1_sol4,
                day1::part1_sol5,
            ],
            real1: include_str!("../input/day1/real1.txt"),
            example1: include_str!("../input/day1/example1.txt"),
            part2s: vec![day1::part2_sol1, day1::part2_sol2, day1::part2_sol3],
            real2: include_str!("../input/day1/real2.txt"),
            example2: include_str!("../input/day1/example2.txt"),
        },
        DayChallenge {
            part1s: vec![day2::part1_sol1],
            real1: include_str!("../input/day2/real1.txt"),
            example1: include_str!("../input/day2/example1.txt"),
            part2s: vec![day2::part2_sol1],
            real2: include_str!("../input/day2/real2.txt"),
            example2: include_str!("../input/day2/example2.txt"),
        },
        DayChallenge {
            part1s: vec![day3::part1_sol1],
            real1: include_str!("../input/day3/real1.txt"),
            example1: include_str!("../input/day3/example1.txt"),
            part2s: vec![day3::part2_sol1],
            real2: include_str!("../input/day3/real2.txt"),
            example2: include_str!("../input/day3/example2.txt"),
        },
    ]
}
