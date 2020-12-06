use anyhow::Result;
use itertools::Itertools;
use std::collections::HashSet;

fn part1(input: &[String]) -> usize {
    input
        .split(|line| line.is_empty())
        .map(|group| {
            group
                .iter()
                .flat_map(|line| line.chars())
                .collect::<HashSet<_>>()
                .len()
        })
        .sum()
}

fn part2(input: &[String]) -> usize {
    input
        .split(|line| line.is_empty())
        .map(|group| {
            let len = group.clone().len();
            let mut group: Box<[char]> = group.concat().chars().collect();
            group.sort();
            group
                .into_iter()
                .group_by(|g| *g)
                .into_iter()
                .map(|(_, q)| q.count())
                .filter(|qlen| *qlen == len)
                .count()
        })
        .sum()
}

fn main() -> Result<()> {
    let input: Vec<String> = common::std_input_vec()?;
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
    Ok(())
}
