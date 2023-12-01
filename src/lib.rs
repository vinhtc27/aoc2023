extern crate core;

pub mod days;

use std::fmt::{Display, Formatter};
use std::time::Instant;

use nom::combinator::opt;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete as character,
    combinator::{all_consuming, map},
    sequence::{pair, preceded},
};
use thiserror::Error;

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
    (I8, i8),
    (Day10Result, Day10Result)
}

#[derive(Debug)]
pub struct Day10Result([u64; 6]);

impl Display for Day10Result {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in self.0 {
            let s: String = (0..40)
                .map(|i| if (1 << i) & line != 0 { '#' } else { ' ' })
                .collect();
            writeln!(f, "{}", s)?
        }
        Ok(())
    }
}

impl From<&'_ str> for Answers {
    fn from(s: &'_ str) -> Self {
        Answers::String(s.to_string())
    }
}

pub trait IntoDayResult {
    fn into_result(self) -> anyhow::Result<DayResult>;
}

impl IntoDayResult for () {
    fn into_result(self) -> anyhow::Result<DayResult> {
        Ok(DayResult { answers: None })
    }
}

impl<A> IntoDayResult for A
where
    A: Into<Answers>,
{
    fn into_result(self) -> anyhow::Result<DayResult> {
        Ok(DayResult {
            answers: Some(self.into()),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct DayResult {
    pub answers: Option<Answers>,
}

pub struct DayEntry {
    pub f1: fn(&'static str) -> anyhow::Result<DayResult>,
    pub real1: &'static str,
    pub example1: &'static str,
    pub f2: fn(&'static str) -> anyhow::Result<DayResult>,
    pub real2: &'static str,
    pub example2: &'static str,
}

pub fn run_day(
    day: u32,
    DayEntry {
        f1,
        real1,
        example1,
        f2,
        real2,
        example2,
    }: &DayEntry,
    is_exaple: bool,
) -> anyhow::Result<()> {
    let input1 = if is_exaple { *example1 } else { *real1 };
    let start1 = Instant::now();
    let result1 = f1(input1)?;
    let end1 = start1.elapsed();
    println!("Day {} - Part 1:", day);
    if let Some(part1) = result1.answers {
        let part1 = format!("{part1}");
        for line in part1.lines() {
            println!("\t{line}");
        }
    }
    println!("Duration:");
    println!("\t{} µs", end1.as_micros());
    println!("\t{} ns", end1.as_nanos());
    println!();

    let input2 = if is_exaple { *example2 } else { *real2 };
    let start2 = Instant::now();
    let result2 = f2(input2)?;
    let end2 = start2.elapsed();
    println!("Day {} - Part 2:", day);
    if let Some(part2) = result2.answers {
        let part2 = format!("{part2}");
        for line in part2.lines() {
            println!("\t{line}");
        }
    }
    println!("Duration:");
    println!("\t{} µs", end2.as_micros());
    println!("\t{} ns", end2.as_nanos());
    println!();

    Ok(())
}

#[derive(Debug, Eq, PartialEq)]
pub enum Runnable {
    Latest,                          // empty
    All,                             // .
    Range { first: u32, last: u32 }, // 12-15
}

impl Runnable {
    pub fn load_all<I: IntoIterator<Item = T>, T: AsRef<str>>(
        source: I,
    ) -> Result<Vec<Runnable>, ConversionError> {
        let mut runnables = Vec::new();
        for cmd in source {
            let cmd = cmd.as_ref();
            let runnable = cmd.try_into()?;
            runnables.push(runnable);
        }
        if runnables.is_empty() {
            runnables.push(Runnable::Latest);
        }
        Ok(runnables)
    }
}

impl<'a> TryFrom<&'a str> for Runnable {
    type Error = ConversionError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let res = parse_runnable(value).map(|r| r.1)?;

        if let Runnable::Range { first, last } = res {
            if first > last {
                return Err(ConversionError::OutOfOrder);
            }
        }

        Ok(res)
    }
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum ConversionError {
    #[error("Input was incomplete")]
    Incomplete,
    #[error("Day range was not increasing")]
    OutOfOrder,
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Parse failure: {0}")]
    ParseFailure(String),
}

impl<T> From<nom::Err<nom::error::Error<T>>> for ConversionError {
    fn from(err: nom::Err<nom::error::Error<T>>) -> Self {
        match err {
            nom::Err::Incomplete(_) => ConversionError::Incomplete,
            nom::Err::Error(error) => ConversionError::ParseError(format!("{:?}", error.code)),
            nom::Err::Failure(failure) => {
                ConversionError::ParseFailure(format!("{:?}", failure.code))
            }
        }
    }
}

fn parse_runnable(input: &str) -> nom::IResult<&str, Runnable> {
    alt((
        map(parse_latest, |_| Runnable::All),
        map(parse_range, |(first, last)| Runnable::Range {
            first,
            last: last.unwrap_or(first),
        }),
    ))(input)
}

fn parse_latest(input: &str) -> nom::IResult<&str, &str> {
    all_consuming(tag("."))(input)
}

fn parse_range(input: &str) -> nom::IResult<&str, (u32, Option<u32>)> {
    all_consuming(pair(
        character::u32,
        opt(preceded(tag("-"), character::u32)),
    ))(input)
}

#[cfg(test)]
mod tests {
    use crate::{parse_runnable, Runnable};

    #[test]
    fn no_args_defaults_to_latest() {
        let runnables = Runnable::load_all::<[&str; 0], _>([]);
        assert_eq!(runnables, Ok(vec![Runnable::Latest]));
    }

    #[test]
    fn dot_arg_is_all() {
        let runnables = Runnable::load_all::<_, _>(["."]);
        assert_eq!(runnables, Ok(vec![Runnable::All]));
    }

    #[test]
    fn parser_handles_latest() {
        assert_eq!(parse_runnable("."), Ok(("", Runnable::All)));
    }
}
