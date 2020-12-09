use anyhow::Result;
use itertools::Itertools;

fn part1(input: &[u64]) -> u64 {
    *input[25..]
        .iter()
        .enumerate()
        .find(|(idx, val)| {
            !input[*idx..*idx + 25]
                .iter()
                .combinations(2)
                .any(|c| c.into_iter().cloned().sum::<u64>() == **val)
        })
        .unwrap()
        .1
}

fn part2(input: &[u64]) -> u64 {
    let target = part1(input);

    let mut low_idx = 0;
    let mut high_idx = 0;
    let mut sum = 0;

    while sum != target {
        if sum < target {
            sum += input[high_idx];
            high_idx += 1;
        } else {
            sum -= input[low_idx];
            low_idx += 1;
        }
    }

    let s = &input[low_idx..high_idx];
    s.iter().min().unwrap() + s.iter().max().unwrap()
}

fn main() -> Result<()> {
    let input = common::std_input_vec()?;
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));

    Ok(())
}
