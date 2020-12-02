use super::Entry;
use anyhow::Error;
use nom::character::complete::{alpha1, anychar, char as ch, digit1, space1};
use nom::combinator::{map, map_opt};
use nom::sequence::{separated_pair, tuple};
use nom::{Finish, IResult};

impl Entry {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let range = map_opt(
            separated_pair(digit1, ch('-'), digit1),
            |(low, high): (&str, &str)| {
                Some(low.parse::<usize>().ok()?..=high.parse::<usize>().ok()?)
            },
        );

        map(
            tuple((range, space1, anychar, ch(':'), space1, alpha1)),
            |(r, _, c, _, _, pass)| Self {
                range: r,
                character: c,
                password: pass.to_owned(),
            },
        )(input)
    }
}

impl std::str::FromStr for Entry {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        let (_, entry) = Self::parse(s).finish().map_err(|err| nom::error::Error {
            input: err.input.to_owned(),
            code: err.code,
        })?;

        Ok(entry)
    }
}
