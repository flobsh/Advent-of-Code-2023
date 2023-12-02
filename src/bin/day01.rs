use aoc_2023;

const PATTERNS: [&str; 20] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1", "2",
    "3", "4", "5", "6", "7", "8", "9",
];

struct Day01;

impl Day01 {
    fn str_to_double_digit_number(input: &str) -> Option<usize> {
        let first_digit = input.chars().find_map(|c| c.to_digit(10));
        let last_digit = input.chars().rev().find_map(|c| c.to_digit(10));

        Some((first_digit? * 10 + last_digit?) as usize)
    }

    fn str_literal_to_double_digit_number(input: &str) -> Option<usize> {
        let first_digit = (0..input.len()).find_map(|i| {
            PATTERNS
                .iter()
                .enumerate()
                .find(|(_, &number)| input[i..].starts_with(number))
                .map(|(j, _)| j % 10)
        });

        let last_digit = (0..input.len()).rev().find_map(|i| {
            PATTERNS
                .iter()
                .enumerate()
                .find(|(_, &number)| input[i..].starts_with(number))
                .map(|(j, _)| j % 10)
        });

        Some(first_digit? * 10 + last_digit?)
    }
}

impl aoc_2023::Day for Day01 {
    type ParsedInput = String;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input(input: &str) -> Result<Self::ParsedInput, aoc_2023::Error> {
        Ok(input.into())
    }

    fn part_1(input: &Self::ParsedInput) -> Result<Self::Output1, aoc_2023::Error> {
        input
            .lines()
            .map(|line| Self::str_to_double_digit_number(line).ok_or(aoc_2023::Error::GenericError))
            .sum()
    }

    fn part_2(input: &Self::ParsedInput) -> Result<Self::Output2, aoc_2023::Error> {
        input
            .lines()
            .map(|line| {
                Self::str_literal_to_double_digit_number(line).ok_or(aoc_2023::Error::GenericError)
            })
            .sum()
    }
}

fn main() -> Result<(), aoc_2023::Error> {
    aoc_2023::run_day::<Day01>(1, "inputs/day01.txt")
}

#[cfg(test)]
mod tests {
    use aoc_2023::Day;

    use crate::Day01;

    #[test]
    fn part_1() {
        let input = std::fs::read_to_string("inputs/tests/day01_p1.txt").unwrap();
        assert_eq!(Day01::part_1(&input).unwrap(), 142)
    }

    #[test]
    fn part_2() {
        let input = std::fs::read_to_string("inputs/tests/day01_p2.txt").unwrap();
        assert_eq!(Day01::part_2(&input).unwrap(), 281)
    }
}
