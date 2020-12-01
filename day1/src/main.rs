use anyhow::Result;
use itertools::Itertools;
use std::io::{self, BufRead};

#[allow(dead_code)]
fn part1() -> Result<()> {
    let stdin = io::stdin();
    let data = stdin
        .lock()
        .lines()
        .map(|line| -> Result<u32> { line?.parse().map_err(Into::into) })
        .collect::<Result<Vec<_>, _>>()?;

    // Buggy - single `1010` would be accepted, even if it shouldnt, but I checked out manually no
    // such thing in input file
    let result = data[..]
        .iter()
        .cartesian_product(data[..].iter())
        .filter(|(l, r)| **l + **r == 2020)
        .map(|(l, r)| l * r)
        .next()
        .unwrap();

    println!("{}", result);

    Ok(())
}

#[allow(dead_code)]
fn part2() -> Result<()> {
    let stdin = io::stdin();
    let data = stdin
        .lock()
        .lines()
        .map(|line| -> Result<u32> { line?.parse().map_err(Into::into) })
        .collect::<Result<Vec<_>, _>>()?;

    // Buggy - single `1010` would be accepted, even if it shouldnt, but I checked out manually no
    // such thing in input file
    let result = data[..]
        .iter()
        .cartesian_product(data[..].iter())
        .cartesian_product(data[..].iter())
        .filter(|((l, r), x)| **l + **r + **x == 2020)
        .map(|((l, r), x)| l * r * x)
        .next()
        .unwrap();

    println!("{}", result);

    Ok(())
}

fn main() -> Result<()> {
    part2()
}
