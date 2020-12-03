use anyhow::{anyhow, Error, Result};

#[derive(Clone, Copy)]
enum Field {
    Empty,
    Tree,
}

struct Line {
    line: Vec<Field>,
}

impl std::str::FromStr for Line {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line = s
            .chars()
            .map(|c| match c {
                '.' => Ok(Field::Empty),
                '#' => Ok(Field::Tree),
                c => return Err(anyhow!("Invalid character: {}", c)),
            })
            .collect::<Result<_, _>>()?;

        Ok(Self { line })
    }
}

impl Line {
    fn get(&self, idx: usize) -> Field {
        self.line[idx % self.line.len()]
    }
}

struct Map {
    map: Vec<Line>,
}

impl Map {
    fn from_stdin() -> Result<Self> {
        common::std_input_vec().map(|map| Self { map })
    }

    fn get(&self, x: usize, y: usize) -> Option<Field> {
        if y >= self.map.len() {
            None
        } else {
            Some(self.map[y].get(x))
        }
    }
}

fn calculate(input: &Map, (slopex, slopey): (usize, usize)) -> usize {
    std::iter::successors(Some((0, 0)), |(x, y)| Some((x + slopex, y + slopey)))
        .map(|(x, y)| input.get(x, y))
        .take_while(Option::is_some)
        .filter(|field| matches!(field, Some(Field::Tree)))
        .count()
}

fn part1(input: &Map) -> usize {
    calculate(input, (3, 1))
}

fn part2(input: &Map) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|slope| calculate(input, *slope))
        .product()
}

fn main() -> Result<()> {
    let input = Map::from_stdin()?;

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));

    Ok(())
}
