use anyhow::{Error, Result};
use std::io;
use std::str::FromStr;

/// Reads input as lines, where every line is parsed to given type.
pub fn input_iter<T, Input>(input: Input) -> impl Iterator<Item = Result<T>>
where
    T: FromStr,
    T::Err: Into<Error>,
    Input: io::BufRead,
{
    input
        .lines()
        .map(|item| -> Result<T> { item?.parse().map_err(Into::into) })
}

/// Reads input as lines, where every line is parsed to given type.
pub fn input_vec<T, Input>(input: Input) -> Result<Vec<T>>
where
    T: FromStr,
    T::Err: Into<Error>,
    Input: io::BufRead,
{
    input_iter(input).collect()
}

/// Reads input as lines, where every line is parsed to given type.
pub fn std_input_vec<T>() -> Result<Vec<T>>
where
    T: FromStr,
    T::Err: Into<Error>,
{
    let input = io::stdin();
    input_iter(input.lock()).collect()
}
