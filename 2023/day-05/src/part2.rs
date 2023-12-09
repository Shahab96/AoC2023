use crate::custom_error::AocError;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{anychar, digit1, line_ending, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{terminated, tuple},
    IResult,
};
use rayon::prelude::*;

fn parse_range_line<'a>(input: &'a str) -> IResult<&'a str, (usize, usize, usize)> {
    tuple((
        map_res(terminated(digit1, space1), |s: &str| s.parse::<usize>()),
        map_res(terminated(digit1, space1), |s: &str| s.parse::<usize>()),
        map_res(digit1, |s: &str| s.parse::<usize>()),
    ))(input)
}

fn parse_mapping(input: &str) -> IResult<&str, Vec<(usize, usize, usize)>> {
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
pub fn process(input: &str) -> miette::Result<usize, AocError> {
    let (input, seeds) = parse_seeds(input).unwrap();
    let seeds = seeds.into_iter().step_by(2).collect::<Vec<_>>();
    let (input, _) = parse_heading(input).unwrap();
    let (input, soil_seed_map) = parse_mapping(input).unwrap();
    let (input, _) = parse_heading(input).unwrap();
    let (input, fertilizer_soil_map) = parse_mapping(input).unwrap();
    let (input, _) = parse_heading(input).unwrap();
    let (input, water_fertilizer_map) = parse_mapping(input).unwrap();
    let (input, _) = parse_heading(input).unwrap();
    let (input, light_water_map) = parse_mapping(input).unwrap();
    let (input, _) = parse_heading(input).unwrap();
    let (input, temperature_light_map) = parse_mapping(input).unwrap();
    let (input, _) = parse_heading(input).unwrap();
    let (input, humidity_temperature_map) = parse_mapping(input).unwrap();
    let (input, _) = parse_heading(input).unwrap();
    let (_, location_humidity_map) = parse_mapping(input).unwrap();

    Ok(seeds
        .par_iter()
        .filter_map(|seed| {
            let soil = soil_seed_map
                .iter()
                .find(|(_, s, r)| (*s..(s + r)).contains(seed))
                .map(|(soil, s, _)| {
                    let difference = seed - s;
                    soil + difference
                })?;

            let fertilizer = fertilizer_soil_map
                .iter()
                .find(|(_, s, r)| (*s..(s + r)).contains(&soil))
                .map(|(fertilizer, s, _)| {
                    let difference = soil - s;
                    fertilizer + difference
                })?;

            let water = water_fertilizer_map
                .iter()
                .find(|(_, f, r)| (*f..(f + r)).contains(&fertilizer))
                .map(|(water, f, _)| {
                    let difference = fertilizer - f;
                    water + difference
                })?;

            let light = light_water_map
                .iter()
                .find(|(_, w, r)| (*w..(w + r)).contains(&water))
                .map(|(light, w, _)| {
                    let difference = water - w;
                    light + difference
                })?;

            let temperature = temperature_light_map
                .iter()
                .find(|(_, l, r)| (*l..(l + r)).contains(&light))
                .map(|(temperature, l, _)| {
                    let difference = light - l;
                    temperature + difference
                })?;

            let humidity = humidity_temperature_map
                .iter()
                .find(|(_, t, r)| (*t..(t + r)).contains(&temperature))
                .map(|(humidity, t, _)| {
                    let difference = temperature - t;
                    humidity + difference
                })?;

            location_humidity_map
                .iter()
                .find(|(_, h, r)| (*h..(h + r)).contains(&humidity))
                .map(|(location, h, _)| {
                    let difference = humidity - h;
                    location + difference
                })
        })
        .min()
        .unwrap())
}
