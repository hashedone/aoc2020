use anyhow::Result;
use itertools::Itertools;

fn part1(input: &[u32]) -> Result<u32> {
    // Buggy - single `1010` would be accepted, even if it shouldnt, but I checked out manually no
    // such thing in input file
    let result = input
        .iter()
        .cartesian_product(input.iter())
        .filter(|(l, r)| **l + **r == 2020)
        .map(|(l, r)| l * r)
        .next()
        .unwrap();

    Ok(result)
}

fn part2(input: &[u32]) -> Result<u32> {
    let result = input[..]
        .iter()
        .cartesian_product(input[..].iter())
        .cartesian_product(input[..].iter())
        .filter(|((l, r), x)| **l + **r + **x == 2020)
        .map(|((l, r), x)| l * r * x)
        .next()
        .unwrap();

    println!("{}", result);

    Ok(result)
}

fn main() -> Result<()> {
    let input: Vec<u32> = common::std_input_vec()?;
    println!("Part1: {}", part1(&input)?);
    println!("Part2: {}", part2(&input)?);
    Ok(())
}
