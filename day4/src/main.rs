use anyhow::{anyhow, Error, Result};
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Clone)]
struct Passport {
    byr: u32,
    iyr: u32,
    eyr: u32,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}

impl Passport {
    fn validate(&self) -> bool {
        let byr = (1920..=2002).contains(&self.byr);
        let iyr = (2010..=2020).contains(&self.iyr);
        let eyr = (2020..=2030).contains(&self.eyr);
        let hgtcm = self.hgt.strip_suffix("cm").map_or(false, |s| {
            s.parse::<u32>()
                .ok()
                .map_or(false, |h| (150..=193).contains(&h))
        });
        let hgtin = self.hgt.strip_suffix("in").map_or(false, |s| {
            s.parse::<u32>()
                .ok()
                .map_or(false, |h| (59..=76).contains(&h))
        });
        let hgt = hgtcm || hgtin;
        let hcl = self.hcl.len() == 7
            && self
                .hcl
                .strip_prefix('#')
                .map_or(false, |s| s.chars().all(|c| "1234567890abcdef".contains(c)));
        let ecl = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&self.ecl.as_str());
        let pid = self.pid.len() == 9 && self.pid.chars().all(|c| c.is_digit(10));
        byr && iyr && eyr && hgt && hcl && ecl && pid
    }
}

impl FromStr for Passport {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: HashMap<&str, &str> = s
            .split(&[' ', '\n'][..])
            .map(|item| {
                let mut split = item.split(':');
                (split.next().unwrap(), split.next().unwrap())
            })
            .collect();

        Ok(Passport {
            byr: data.get("byr").ok_or(anyhow!("byr"))?.parse()?,
            iyr: data.get("iyr").ok_or(anyhow!("iyr"))?.parse()?,
            eyr: data.get("eyr").ok_or(anyhow!("eyr"))?.parse()?,
            hgt: (*data.get("hgt").ok_or(anyhow!("hgt"))?).to_owned(),
            hcl: (*data.get("hcl").ok_or(anyhow!("hcl"))?).to_owned(),
            ecl: (*data.get("ecl").ok_or(anyhow!("ecl"))?).to_owned(),
            pid: (*data.get("pid").ok_or(anyhow!("pid"))?).to_owned(),
            cid: data.get("cid").map(|cid| (*cid).to_owned()),
        })
    }
}

fn read_input() -> Result<Vec<Result<Passport>>> {
    Ok(common::std_input_vec()?
        .split(|line: &String| line.is_empty())
        .map(|group| group.join(" ").parse())
        .collect())
}

fn part1(input: &[Result<Passport>]) -> usize {
    input.into_iter().filter(|pass| pass.is_ok()).count()
}

fn part2(input: &[Result<Passport>]) -> usize {
    input
        .into_iter()
        .filter(|pass| match pass {
            Ok(pass) => pass.validate(),
            _ => false,
        })
        .count()
}

fn main() -> Result<()> {
    let input = read_input()?;
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));

    Ok(())
}
