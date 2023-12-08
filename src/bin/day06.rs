use aoc_2023;

struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn winning_time_bounds(&self) -> (usize, usize) {
        let (is_sqrt_exact, delta_sqrt) = Day06::isqrt(self.time.pow(2) - 4 * self.distance);

        let (is_lb_div_exact, lower_bound) = Day06::div_and_mod(self.time - delta_sqrt, 2);
        let (is_ub_div_exact, upper_bound) = Day06::div_and_mod(self.time + delta_sqrt, 2);

        let lower_bound_correction = if is_sqrt_exact || !is_lb_div_exact {
            1
        } else {
            0
        };

        let upper_bound_correction = if is_sqrt_exact && is_ub_div_exact {
            1
        } else {
            0
        };

        (
            lower_bound + lower_bound_correction,
            upper_bound - upper_bound_correction,
        )
    }
}

struct Day06;

impl Day06 {
    fn isqrt(n: usize) -> (bool, usize) {
        if n <= 1 {
            return (true, n);
        }

        let mut x0 = n / 2;
        let mut x1 = (x0 + n / x0) / 2;

        while x1 < x0 {
            x0 = x1;
            x1 = (x0 + n / x0) / 2
        }

        (x0 * x0 == n, x0)
    }

    fn div_and_mod(n: usize, rhs: usize) -> (bool, usize) {
        (n % rhs == 0, n / rhs)
    }
}

impl aoc_2023::Day for Day06 {
    type ParsedInput = Vec<Race>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input(input: &str) -> Result<Self::ParsedInput, aoc_2023::Error> {
        let parse_line_to_numbers = |line: &str| -> Result<Vec<usize>, aoc_2023::Error> {
            line.split_once(':')
                .ok_or(aoc_2023::Error::GenericError)
                .and_then(|(_, times)| {
                    Ok(times
                        .split_whitespace()
                        .map(|t| t.trim().parse())
                        .collect::<Result<_, _>>()?)
                })
        };

        let mut lines = input.lines();
        let times = lines
            .next()
            .ok_or(aoc_2023::Error::GenericError)
            .and_then(parse_line_to_numbers)?;
        let distances = lines
            .next()
            .ok_or(aoc_2023::Error::GenericError)
            .and_then(parse_line_to_numbers)?;

        Ok(times
            .iter()
            .zip(distances.iter())
            .map(|(t, d)| Race {
                time: *t,
                distance: *d,
            })
            .collect())
    }

    fn part_1(input: &Self::ParsedInput) -> Result<Self::Output1, aoc_2023::Error> {
        Ok(input
            .iter()
            .map(|race| race.winning_time_bounds())
            .map(|(lower_bound, upper_bound)| upper_bound - lower_bound + 1)
            .product())
    }

    fn part_2(input: &Self::ParsedInput) -> Result<Self::Output2, aoc_2023::Error> {
        todo!()
    }
}

fn main() -> Result<(), aoc_2023::Error> {
    aoc_2023::run_day::<Day06>(6, "inputs/day06.txt")
}

#[cfg(test)]
mod tests {
    use crate::Day06;
    use aoc_2023::Day;

    #[test]
    fn sqrt_non_exact() {
        assert_eq!(Day06::isqrt(13), (false, 3))
    }

    #[test]
    fn sqrt_exact() {
        assert_eq!(Day06::isqrt(100), (true, 10))
    }

    #[test]
    fn part_1() {
        let input = std::fs::read_to_string("inputs/tests/day06.txt").unwrap();
        let parsed_input = Day06::parse_input(&input).unwrap();
        assert_eq!(Day06::part_1(&parsed_input).unwrap(), 288);
    }
}
