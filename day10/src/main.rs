use anyhow::Result;

fn part1(input: &[u64]) -> u64 {
    let (_, ones, threes) =
        input
            .iter()
            .fold((0, 0, 0), |(last, ones, threes), next| match next - last {
                1 => (*next, ones + 1, threes),
                2 => (*next, ones, threes),
                3 => (*next, ones, threes + 1),
                _ => panic!("Too much differrence"),
            });
    ones * (threes + 1)
}

fn part2(input: &[u64]) -> u64 {
    input
        .iter()
        .fold([(0, 0), (0, 0), (0, 1)], |history, next| {
            let cnt = history
                .iter()
                .filter(|(v, _)| next - v <= 3)
                .map(|(_, cnt)| cnt)
                .sum();
            [history[1], history[2], (*next, cnt)]
        })[2]
        .1
}

fn main() -> Result<()> {
    let mut input = common::std_input_vec()?;
    input.sort();
    println!("{}", input.len());
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));

    Ok(())
}
