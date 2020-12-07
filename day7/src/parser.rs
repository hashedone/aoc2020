use super::Bag;
use anyhow::Error;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char as ch, digit1};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::{Finish, IResult};
use std::collections::HashMap;

fn color(input: &str) -> IResult<&str, String> {
    let (input, (desc, _, color)) = tuple((alpha1, ch(' '), alpha1))(input)?;
    Ok((input, format!("{} {}", desc, color)))
}

fn content_item(input: &str) -> IResult<&str, (String, usize)> {
    let single = map(tuple((tag("1 "), color, tag(" bag"))), |(_, color, _)| {
        (color, 1)
    });
    let multi = map(
        tuple((digit1::<&str, _>, ch(' '), color, tag(" bags"))),
        |(cnt, _, color, _)| {
            let cnt: usize = cnt.parse().unwrap();
            (color.to_owned(), cnt)
        },
    );

    alt((single, multi))(input)
}

fn content(input: &str) -> IResult<&str, HashMap<String, usize>> {
    let non_empty = map(separated_list1(tag(", "), content_item), |data| {
        data.into_iter().collect()
    });
    let empty = map(tag("no other bags"), |_| HashMap::new());

    alt((non_empty, empty))(input)
}

impl Bag {
    pub fn parse(input: &str) -> IResult<&str, Bag> {
        let (input, (color, _, content, _)) =
            tuple((color, tag(" bags contain "), content, ch('.')))(input)?;
        Ok((
            input,
            Bag {
                color,
                childs: content,
            },
        ))
    }
}

impl std::str::FromStr for Bag {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        let (_, entry) = Self::parse(s).finish().map_err(|err| nom::error::Error {
            input: err.input.to_owned(),
            code: err.code,
        })?;

        Ok(entry)
    }
}
