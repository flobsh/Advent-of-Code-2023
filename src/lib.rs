mod error;
pub use error::Error;

pub trait Day {
    type ParsedInput;
    type Output1: std::fmt::Display;
    type Output2: std::fmt::Display;

    fn parse_input(input: &str) -> Result<Self::ParsedInput, Error>;
    fn part_1(input: &Self::ParsedInput) -> Result<Self::Output1, Error>;
    fn part_2(input: &Self::ParsedInput) -> Result<Self::Output2, Error>;
}

pub fn run_day<T: Day>(day_number: usize, input: &str) -> Result<(), Error> {
    println!("==== Day {day_number:02} ====");

    let parsed_input = T::parse_input(&std::fs::read_to_string(input)?)?;
    println!("Part 1 solution: {}", T::part_1(&parsed_input)?);
    println!("Part 2 solution: {}", T::part_2(&parsed_input)?);

    Ok(())
}
