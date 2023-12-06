use aoc2023::run_day_challenge;
use aoc2023::{days, DayChallenge};

fn main() -> anyhow::Result<()> {
    let is_test = std::env::var_os("TEST").is_some();
    let _ = get_days()
        .into_iter()
        .try_for_each(|day_challenge| run_day_challenge(&day_challenge, is_test))?;

    Ok(())
}

fn get_days() -> Vec<DayChallenge> {
    use days::day6;
    vec![
        // DayChallenge {
        //     day: 2,
        //     part1s: vec![day2::part1_sol1, day2::part1_sol2],
        //     part2s: vec![day2::part2_sol1, day2::part2_sol2],
        // },
        DayChallenge {
            day: 6,
            part1s: vec![day6::part1_sol1, day6::part1_sol2],
            part2s: vec![day6::part2_sol1, day6::part2_sol2],
        },
    ]
}
