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

pub fn run_challenge<T: Day>(
    input: &str,
    part_1_prompt: &str,
    part_2_prompt: &str,
) -> Result<(), Error> {
    let parsed_input = T::parse_input(input)?;
    println!("Part 1 {part_1_prompt}: {}", T::part_1(&parsed_input)?);
    println!("Part 2 {part_2_prompt}: {}", T::part_2(&parsed_input)?);

    Ok(())
}
