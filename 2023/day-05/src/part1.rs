use std::collections::HashMap;

use crate::custom_error::AocError;
use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{anychar, digit1, line_ending, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{terminated, tuple},
    IResult,
};

fn parse_range_line<'a>(input: &'a str) -> IResult<&'a str, HashMap<u32, u32>> {
    let (input, (destination, source, range)) = tuple((
        map_res(terminated(digit1, space1), |s: &str| s.parse::<usize>()),
        map_res(terminated(digit1, space1), |s: &str| s.parse::<usize>()),
        map_res(digit1, |s: &str| s.parse::<usize>()),
    ))(input)?;

    let destination = destination..range;
    let source = source..range;

    let destination = destination.collect_vec();
    let source = source.collect_vec();

    let map = source
        .iter()
        .zip(destination.iter())
        .map(|(src, dst)| (*src as u32, *dst as u32))
        .collect::<HashMap<u32, u32>>();

    Ok((input, map))
}

fn parse_mapping(input: &str) -> IResult<&str, Vec<HashMap<u32, u32>>> {
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
    // let seed_to_soil_map: HashMap<u32, u32> = mapping
    //     .into_iter()
    //     .flat_map(|map| map.into_iter())
    //     .collect();
    let (input, _) = parse_heading(input).unwrap();
    let (input, mapping) = parse_mapping(input).unwrap();
    // let soil_to_fertilizer_map: HashMap<u32, u32> = mapping
    //     .into_iter()
    //     .flat_map(|map| map.into_iter())
    //     .collect();
    let (input, _) = parse_heading(input).unwrap();
    let (input, mapping) = parse_mapping(input).unwrap();
    // let fertilizer_to_water_map: HashMap<u32, u32> = mapping
    //     .into_iter()
    //     .flat_map(|map| map.into_iter())
    //     .collect();
    let (input, _) = parse_heading(input).unwrap();
    let (input, mapping) = parse_mapping(input).unwrap();
    // let water_to_light_map: HashMap<u32, u32> = mapping
    //     .into_iter()
    //     .flat_map(|map| map.into_iter())
    //     .collect();
    let (input, _) = parse_heading(input).unwrap();
    let (input, mapping) = parse_mapping(input).unwrap();
    // let light_to_temperature_map: HashMap<u32, u32> = mapping
    //     .into_iter()
    //     .flat_map(|map| map.into_iter())
    //     .collect();
    let (input, _) = parse_heading(input).unwrap();
    let (input, mapping) = parse_mapping(input).unwrap();
    let temperature_to_humidity_map: HashMap<u32, u32> = mapping
        .into_iter()
        .flat_map(|map| map.into_iter())
        .collect();
    // let (input, _) = parse_heading(input).unwrap();
    // let (_, mapping) = parse_mapping(input).unwrap();
    // let humidity_to_location_map: HashMap<u32, u32> = mapping
    //     .into_iter()
    //     .flat_map(|map| map.into_iter())
    //     .collect();

    // let mut seed_to_location_map = HashMap::<usize, usize>::new();
    //
    // seeds.iter().for_each(|seed| {
    //     let soil = seed_to_soil_map.get(seed).unwrap();
    //     let fertilizer = soil_to_fertilizer_map.get(soil).unwrap();
    //     let water = fertilizer_to_water_map.get(fertilizer).unwrap();
    //     let light = water_to_light_map.get(water).unwrap();
    //     let temperature = light_to_temperature_map.get(light).unwrap();
    //     let humidity = temperature_to_humidity_map.get(temperature).unwrap();
    //     let location = humidity_to_location_map.get(humidity).unwrap();
    //
    //     // seed_to_location_map.insert(*seed, *location);
    //     println!(
    //         "seed: {}, location: {}, humidity: {}, temperature: {}, light: {}, water: {}, fertilizer: {}, soil: {}",
    //         seed, location, humidity, temperature, light, water, fertilizer, soil
    //     );
    // });

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
