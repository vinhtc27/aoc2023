extern crate core;

pub mod days;

use std::fmt::Display;
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

pub struct DayChallenge {
    pub part1s: Vec<fn(&'static str) -> anyhow::Result<Answers>>,
    pub real1: &'static str,
    pub example1: &'static str,
    pub part2s: Vec<fn(&'static str) -> anyhow::Result<Answers>>,
    pub real2: &'static str,
    pub example2: &'static str,
}

pub fn run_day_challenge(
    day: usize,
    DayChallenge {
        part1s,
        real1,
        example1,
        part2s,
        real2,
        example2,
    }: &DayChallenge,
    is_exaple: bool,
) -> anyhow::Result<()> {
    let input1 = if is_exaple { *example1 } else { *real1 };
    println!("Day {} - Part 1:", day);
    for (sol, part) in part1s.iter().enumerate() {
        let now = Instant::now();
        let answers = part(input1)?;
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
    let input2 = if is_exaple { *example2 } else { *real2 };
    println!("Day {} - Part 2:", day);
    for (sol, part) in part2s.iter().enumerate() {
        let now = Instant::now();
        let answers = part(input2)?;
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
