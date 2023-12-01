use aoc2023::{days, DayEntry};
use aoc2023::{run_day, Runnable};

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let is_test = std::env::var_os("TEST").is_some();

    let days = get_days();

    let runnables =
        Runnable::load_all(std::env::args().skip(1)).context("failed to parse runnables")?;

    for runnable in runnables {
        let days_to_run = match runnable {
            Runnable::Latest => {
                let day = days.len() as u32;
                day..=day
            }
            Runnable::All => {
                let last = days.len() as u32;
                1..=last
            }
            Runnable::Range { first, last } => first..=last,
        };
        days_to_run
            .into_iter()
            .try_for_each(|day| run_day(day, &days[(day - 1) as usize], is_test))?;
    }

    Ok(())
}

fn get_days() -> Vec<DayEntry> {
    vec![DayEntry {
        f1: days::day01::part1,
        real1: include_str!("../input/day1/real1.txt"),
        example1: include_str!("../input/day1/example1.txt"),
        f2: days::day01::part2,
        real2: include_str!("../input/day1/real2.txt"),
        example2: include_str!("../input/day1/example2.txt"),
    }]
}
