extern crate core;

pub mod days;

use std::fmt::Display;
use std::fs;
use std::time::Instant;

macro_rules! impl_answer_enum {
    ( $( ($variant:tt, $ty:ty) ),* ) => {
        #[derive(Debug)]
        pub enum Answers {
            $(
                $variant($ty),
            )*
        }

        $(
            impl From<$ty> for Answers {
                fn from(t: $ty) -> Self {
                    Answers::$variant(t)
                }
            }
        )*

        // assumes all types impl Display
        impl Display for Answers {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        Answers::$variant(t) => write!(f, "{t}"),
                    )*
                }
            }
        }

        impl Eq for Answers {}

        impl PartialEq for Answers {
            fn eq(&self, other: &Self) -> bool {
                let val_self = match self {
                    $(
                    Answers::$variant(v) => format!("{v}"),
                    )*
                };
                let val_other = match other {
                    $(
                    Answers::$variant(v) => format!("{v}"),
                    )*
                };
                val_self == val_other
            }
        }
    }
}

impl_answer_enum! {
    (String, String),
    (Usize, usize),
    (U64, u64),
    (U32, u32),
    (U16, u16),
    (U8, u8),
    (Isize, isize),
    (I64, i64),
    (I32, i32),
    (I16, i16),
    (I8, i8)
}

impl From<&'_ str> for Answers {
    fn from(s: &'_ str) -> Self {
        Answers::String(s.to_string())
    }
}

pub trait IntoAnswers {
    fn into_answer(self) -> anyhow::Result<Answers>;
}

impl IntoAnswers for () {
    fn into_answer(self) -> anyhow::Result<Answers> {
        Ok(Answers::String("Empty unit () answers".to_string()))
    }
}

impl<A> IntoAnswers for A
where
    A: Into<Answers>,
{
    fn into_answer(self) -> anyhow::Result<Answers> {
        Ok(self.into())
    }
}

type Solutions = Vec<fn(&str) -> anyhow::Result<Answers>>;
pub struct DayChallenge {
    pub day: u8,
    pub part1s: Solutions,
    pub part2s: Solutions,
}

pub fn run_day_challenge(
    DayChallenge {
        day,
        part1s,
        part2s,
    }: &DayChallenge,
    is_exaple: bool,
) -> anyhow::Result<()> {
    let input1 = if is_exaple {
        fs::read_to_string(format!("./input/day{}/example1.txt", day))?
    } else {
        fs::read_to_string(format!("./input/day{}/real1.txt", day))?
    };
    println!("Day {} - Part 1:", day);
    for (sol, part) in part1s.iter().enumerate() {
        let now = Instant::now();
        let answers = part(input1.as_str())?;
        let duration = now.elapsed();
        print!("+ Solution {} | Output: ", sol + 1);
        for line in answers.to_string().lines() {
            print!("{line} ");
        }
        println!(
            "- Duration {} µs - {} ns",
            duration.as_micros(),
            duration.as_nanos()
        );
    }
    let input2 = if is_exaple {
        fs::read_to_string(format!("./input/day{}/example2.txt", day))?
    } else {
        fs::read_to_string(format!("./input/day{}/real2.txt", day))?
    };
    println!("Day {} - Part 2:", day);
    for (sol, part) in part2s.iter().enumerate() {
        let now = Instant::now();
        let answers = part(input2.as_str())?;
        let duration = now.elapsed();
        print!("+ Solution {} | Output: ", sol + 1);
        for line in answers.to_string().lines() {
            print!("{line} ");
        }
        println!(
            "- Duration {} µs - {} ns",
            duration.as_micros(),
            duration.as_nanos()
        );
    }
    println!("____________________________________________________________");
    Ok(())
}
