use std::str::FromStr;

use aoc_2023;

struct Rule {
    source: isize,
    destination: isize,
    range: isize,
}

impl FromStr for Rule {
    type Err = aoc_2023::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_to_isize = |num: Option<&str>| -> Result<isize, Self::Err> {
            let num = num.ok_or(aoc_2023::Error::GenericError)?;
            Ok(num.trim().parse()?)
        };

        let mut rule_iter = s.split_whitespace();

        let destination = parse_to_isize(rule_iter.next());
        let source = parse_to_isize(rule_iter.next());
        let range = parse_to_isize(rule_iter.next());

        Ok(Rule {
            source: source?,
            destination: destination?,
            range: range?,
        })
    }
}

struct Map {
    ruleset: Vec<Rule>,
}

impl Map {
    fn map(&self, input: isize) -> isize {
        self.ruleset
            .iter()
            .find(|rule| input >= rule.source && input <= rule.source + rule.range)
            .map(|rule| input - (rule.source - rule.destination))
            .unwrap_or(input)
    }
}

impl FromStr for Map {
    type Err = aoc_2023::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Map {
            ruleset: s
                .lines()
                .skip(1)
                .map(|rule| rule.parse::<Rule>())
                .collect::<Result<_, _>>()?,
        })
    }
}

struct Day05;

impl aoc_2023::Day for Day05 {
    type ParsedInput = (Vec<isize>, Vec<Map>);
    type Output1 = isize;
    type Output2 = isize;

    fn parse_input(input: &str) -> Result<Self::ParsedInput, aoc_2023::Error> {
        let (seeds, maps) = input
            .split_once("\n\n")
            .ok_or(aoc_2023::Error::GenericError)?;

        let seeds = seeds
            .split_once(':')
            .ok_or(aoc_2023::Error::GenericError)
            .and_then(|(_, seeds)| {
                Ok(seeds
                    .split_whitespace()
                    .map(|seed| seed.trim().parse())
                    .collect::<Result<_, _>>()?)
            });

        let maps = maps
            .split("\n\n")
            .map(|map| map.parse::<Map>())
            .collect::<Result<_, _>>();

        Ok((seeds?, maps?))
    }

    fn part_1(input: &Self::ParsedInput) -> Result<Self::Output1, aoc_2023::Error> {
        let (seeds, maps) = input;

        seeds
            .iter()
            .map(|seed| maps.iter().fold(*seed, |category, map| map.map(category)))
            .min()
            .ok_or(aoc_2023::Error::GenericError)
    }

    fn part_2(input: &Self::ParsedInput) -> Result<Self::Output2, aoc_2023::Error> {
        todo!()
    }
}

fn main() -> Result<(), aoc_2023::Error> {
    aoc_2023::run_day::<Day05>(5, "inputs/day05.txt")
}

#[cfg(test)]
mod tests {
    use crate::Day05;
    use aoc_2023::Day;

    #[test]
    fn part_1() {
        let input = std::fs::read_to_string("inputs/tests/day05.txt").unwrap();
        let parsed_input = Day05::parse_input(&input).unwrap();
        assert_eq!(Day05::part_1(&parsed_input).unwrap(), 35);
    }
}
