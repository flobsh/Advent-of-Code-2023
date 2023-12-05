use std::{collections::HashSet, str::FromStr};

use aoc_2023;

struct Card {
    winning_numbers: HashSet<usize>,
    canditate_numbers: HashSet<usize>,
}

impl Card {
    fn matching_numbers(&self) -> usize {
        self.winning_numbers
            .intersection(&self.canditate_numbers)
            .count()
    }
}

impl FromStr for Card {
    type Err = aoc_2023::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_numbers = |numbers: &str| -> Result<HashSet<usize>, aoc_2023::Error> {
            Ok(numbers
                .split_whitespace()
                .map(|number| number.trim().parse())
                .collect::<Result<_, _>>()?)
        };

        let (_, numbers) = s.split_once(':').ok_or(aoc_2023::Error::GenericError)?;

        let (winnings, candidates) = numbers
            .split_once('|')
            .ok_or(aoc_2023::Error::GenericError)?;

        Ok(Self {
            winning_numbers: parse_numbers(winnings)?,
            canditate_numbers: parse_numbers(candidates)?,
        })
    }
}

struct Day04;

impl aoc_2023::Day for Day04 {
    type ParsedInput = Vec<Card>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input(input: &str) -> Result<Self::ParsedInput, aoc_2023::Error> {
        input.lines().map(|card| card.parse::<Card>()).collect()
    }

    fn part_1(input: &Self::ParsedInput) -> Result<Self::Output1, aoc_2023::Error> {
        Ok(input
            .iter()
            .map(|card| 1 << card.matching_numbers() >> 1)
            .sum())
    }

    fn part_2(input: &Self::ParsedInput) -> Result<Self::Output2, aoc_2023::Error> {
        todo!()
    }
}

fn main() -> Result<(), aoc_2023::Error> {
    aoc_2023::run_day::<Day04>(4, "inputs/day04.txt")
}

#[cfg(test)]
mod tests {
    use aoc_2023::Day;

    use crate::Day04;

    #[test]
    fn part_1() {
        let input = std::fs::read_to_string("inputs/tests/day04_p1.txt").unwrap();
        let parsed_input = Day04::parse_input(&input).unwrap();
        assert_eq!(Day04::part_1(&parsed_input).unwrap(), 13);
    }
}
