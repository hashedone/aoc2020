use anyhow::{anyhow, Result};

fn place(ticket: &str) -> Result<u32> {
    ticket.chars().try_fold(0, |acc, c| match c {
        'F' | 'L' => Ok(acc << 1),
        'B' | 'R' => Ok((acc << 1) + 1),
        _ => Err(anyhow!("Invalid character: {}", c)),
    })
}

fn part1(input: &[String]) -> u32 {
    input.iter().map(|t| place(&t).unwrap()).max().unwrap()
}

fn part2(input: &[String]) -> u32 {
    let mut seats: Vec<_> = input.iter().map(|t| place(&t).unwrap()).collect();
    seats.sort();
    let (prev, _) = seats
        .iter()
        .zip(seats.iter().skip(1))
        .find(|(prev, next)| **next - **prev == 2)
        .unwrap();
    prev + 1
}

fn main() -> Result<()> {
    let input = common::std_input_vec()?;
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));

    Ok(())
}
