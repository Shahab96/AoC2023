use std::{collections::HashMap, ops::Range};

use crate::custom_error::AocError;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{anychar, digit1, line_ending, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{terminated, tuple},
    IResult,
};

fn parse_range_line<'a>(input: &'a str) -> IResult<&'a str, (Range<usize>, Range<usize>)> {
    let (input, (dest, source, range)) = tuple((
        map_res(terminated(digit1, space1), |s: &str| s.parse::<usize>()),
        map_res(terminated(digit1, space1), |s: &str| s.parse::<usize>()),
        map_res(digit1, |s: &str| s.parse::<usize>()),
    ))(input)?;

    Ok((input, (dest..range, source..range)))
}

fn parse_mapping(input: &str) -> IResult<&str, Vec<(Range<usize>, Range<usize>)>> {
    separated_list1(line_ending, parse_range_line)(input)
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds) =
        separated_list1(tag(" "), map_res(digit1, |s: &str| s.parse::<usize>()))(input)?;

    Ok((input, seeds))
}

fn parse_heading(input: &str) -> IResult<&str, ()> {
    let (input, _) = tag("\n\n")(input)?;
    let (input, _) = take_until("\n")(input)?;
    let (input, _) = anychar(input)?;

    Ok((input, ()))
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (input, seeds) = parse_seeds(input).unwrap();
    let (input, _) = parse_heading(input).unwrap();
    let (input, mapping) = parse_mapping(input).unwrap();
    let seed_to_soil_map = mapping.into_iter().collect::<HashMap<_, _>>();
    let (input, _) = parse_heading(input).unwrap();
    let (input, mapping) = parse_mapping(input).unwrap();
    let soil_to_fertilizer_map = mapping.into_iter().collect::<HashMap<_, _>>();
    let (input, _) = parse_heading(input).unwrap();
    let (input, mapping) = parse_mapping(input).unwrap();
    let fertilizer_to_water_map = mapping.into_iter().collect::<HashMap<_, _>>();
    let (input, _) = parse_heading(input).unwrap();
    let (input, mapping) = parse_mapping(input).unwrap();
    let water_to_light_map = mapping.into_iter().collect::<HashMap<_, _>>();
    let (input, _) = parse_heading(input).unwrap();
    let (input, mapping) = parse_mapping(input).unwrap();
    let light_to_temperature_map = mapping.into_iter().collect::<HashMap<_, _>>();
    let (input, _) = parse_heading(input).unwrap();
    let (input, mapping) = parse_mapping(input).unwrap();
    let temperature_to_humidity_map = mapping.into_iter().collect::<HashMap<_, _>>();
    let (input, _) = parse_heading(input).unwrap();
    let (_, mapping) = parse_mapping(input).unwrap();
    let humidity_to_location_map = mapping.into_iter().collect::<HashMap<_, _>>();

    let mut seed_to_location_map = HashMap::<usize, usize>::new();

    for seed in seeds {}

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        todo!("haven't built test yet");
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
