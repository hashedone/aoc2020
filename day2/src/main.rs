use anyhow::Result;

#[derive(Debug)]
struct Entry {
    range: std::ops::RangeInclusive<usize>,
    character: char,
    password: String,
}

mod parser;

fn part1(input: &[Entry]) -> usize {
    input
        .iter()
        .filter(|entry| {
            let cnt = entry
                .password
                .chars()
                .filter(|c| *c == entry.character)
                .count();
            entry.range.contains(&cnt)
        })
        .count()
}

fn part2(input: &[Entry]) -> usize {
    input
        .iter()
        .filter(|entry| {
            let low = *entry.range.start() - 1;
            let high = *entry.range.end() - 1;
            let low = entry
                .password
                .get(low..)
                .map(|s| s.starts_with(entry.character));
            let high = entry
                .password
                .get(high..)
                .map(|s| s.starts_with(entry.character));

            match (low, high) {
                (Some(true), Some(false)) => true,
                (Some(false), Some(true)) => true,
                _ => false,
            }
        })
        .count()
}

fn main() -> Result<()> {
    let input: Vec<Entry> = common::std_input_vec()?;
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));

    Ok(())
}
