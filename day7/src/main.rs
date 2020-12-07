use anyhow::Result;
use std::collections::{HashMap, VecDeque};

mod parser;

#[derive(Debug)]
struct Bag {
    color: String,
    // color => count
    childs: HashMap<String, usize>,
}

#[derive(Debug, Default, Clone)]
struct BagWithIndirect {
    // color => count, direct and indirect childs
    childs: HashMap<String, usize>,
}

fn build_mappings(bags: impl Iterator<Item = Bag>) -> HashMap<String, BagWithIndirect> {
    let mut direct_only: HashMap<_, _> = bags.map(|bag| (bag.color, bag.childs)).collect();
    let mut queue: VecDeque<_> = direct_only
        .iter()
        .map(|(color, _)| color.to_owned())
        .collect();
    let mut with_indirect: HashMap<String, BagWithIndirect> = HashMap::new();

    while let Some(bag) = queue.pop_front() {
        if with_indirect.get(&bag).is_some() {
            continue;
        }

        let directs = direct_only.entry(bag.to_owned()).or_default();
        if directs
            .keys()
            .all(|color| with_indirect.get(color).is_some())
        {
            let mut childs = directs.clone();

            for (child, child_cnt) in directs.iter() {
                for (color, cnt) in with_indirect.get(child).unwrap().childs.iter() {
                    *childs.entry(color.clone()).or_default() += cnt * child_cnt;
                }
            }

            with_indirect.insert(bag, BagWithIndirect { childs });
        } else {
            queue.push_back(bag)
        }
    }

    with_indirect
}

fn part1(data: &HashMap<String, BagWithIndirect>) -> usize {
    data.values()
        .filter(|mapping| mapping.childs.get("shiny gold").is_some())
        .count()
}

fn part2(data: &HashMap<String, BagWithIndirect>) -> usize {
    data.get("shiny gold")
        .unwrap()
        .childs
        .iter()
        .map(|(_, cnt)| *cnt)
        .sum()
}

fn main() -> Result<()> {
    let input: Vec<Bag> = common::std_input_vec()?;
    let data = build_mappings(input.into_iter());

    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));

    Ok(())
}
