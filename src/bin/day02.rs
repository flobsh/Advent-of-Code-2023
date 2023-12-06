use std::str::FromStr;

use aoc_2023;

#[derive(Debug, Default, PartialEq, Eq)]
struct Set {
    red: usize,
    green: usize,
    blue: usize,
}

impl Set {
    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

impl FromStr for Set {
    type Err = aoc_2023::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut set = Set::default();

        for (count, color) in s.split(',').flat_map(|s| s.trim().split_once(' ')) {
            let count = count.trim().parse::<usize>()?;

            match color {
                "red" => set.red += count,
                "green" => set.green += count,
                "blue" => set.blue += count,
                _ => return Err(aoc_2023::Error::GenericError),
            }
        }

        Ok(set)
    }
}

struct Game {
    id: usize,
    sets: Vec<Set>,
}

impl Game {
    fn fewest_cubes_set(&self) -> Set {
        let mut fewest_cubes_set = Set::default();

        for set in self.sets.iter() {
            if set.red > fewest_cubes_set.red {
                fewest_cubes_set.red = set.red
            }
            if set.green > fewest_cubes_set.green {
                fewest_cubes_set.green = set.green
            }
            if set.blue > fewest_cubes_set.blue {
                fewest_cubes_set.blue = set.blue
            }
        }

        fewest_cubes_set
    }
}

impl FromStr for Game {
    type Err = aoc_2023::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once(':')
            .ok_or(aoc_2023::Error::GenericError)
            .and_then(|(game, sets)| {
                Ok(Game {
                    id: game
                        .split_once(' ')
                        .ok_or(aoc_2023::Error::GenericError)
                        .and_then(|(_, id)| Ok(id.trim().parse()?))?,
                    sets: sets
                        .split(';')
                        .map(|set| set.trim().parse::<Set>())
                        .collect::<Result<_, _>>()?,
                })
            })
    }
}

struct Day02;

impl aoc_2023::Day for Day02 {
    type ParsedInput = Vec<Game>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input(input: &str) -> Result<Self::ParsedInput, aoc_2023::Error> {
        input
            .lines()
            .map(|game| game.trim().parse::<Game>())
            .collect::<Result<_, _>>()
    }

    fn part_1(input: &Self::ParsedInput) -> Result<Self::Output1, aoc_2023::Error> {
        let initial_set = Set {
            red: 12,
            green: 13,
            blue: 14,
        };

        Ok(input
            .iter()
            .filter(|game| {
                game.sets.iter().all(|set| {
                    set.red <= initial_set.red
                        && set.green <= initial_set.green
                        && set.blue <= initial_set.blue
                })
            })
            .map(|game| game.id)
            .sum())
    }

    fn part_2(input: &Self::ParsedInput) -> Result<Self::Output2, aoc_2023::Error> {
        Ok(input
            .iter()
            .map(|game| game.fewest_cubes_set().power())
            .sum())
    }
}

fn main() -> Result<(), aoc_2023::Error> {
    aoc_2023::run_day::<Day02>(2, "inputs/day02.txt")
}

#[cfg(test)]
mod tests {
    use aoc_2023::Day;

    use crate::{Day02, Set};

    #[test]
    fn set_from_str() {
        assert_eq!(
            "3 blue, 4 red".parse::<Set>().unwrap(),
            Set {
                red: 4,
                green: 0,
                blue: 3
            }
        )
    }

    #[test]
    fn part_1() {
        let input = std::fs::read_to_string("inputs/tests/day02.txt").unwrap();
        assert_eq!(
            Day02::part_1(&Day02::parse_input(&input).unwrap()).unwrap(),
            8
        )
    }

    #[test]
    fn part_2() {
        let input = std::fs::read_to_string("inputs/tests/day02.txt").unwrap();
        assert_eq!(
            Day02::part_2(&Day02::parse_input(&input).unwrap()).unwrap(),
            2286
        )
    }
}
