use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space0, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
};

fn get_card_value(winning: &[u32], ours: &[u32]) -> u32 {
    let mut power = 0;

    for &number in ours {
        if winning.contains(&number) {
            power += 1;
        }
    }

    match power {
        0 => 0,
        _ => 2u32.pow(power - 1),
    }
}

fn parse_winning_numbers(input: &str) -> IResult<&str, u32> {
    let (input, (winning, ours)) = separated_pair(
        separated_list1(
            space1,
            map_res(preceded(space0, digit1), |s: &str| s.parse::<u32>()),
        ),
        tag(" | "),
        separated_list1(
            space1,
            map_res(preceded(space0, digit1), |s: &str| s.parse::<u32>()),
        ),
    )(input)?;

    let power = get_card_value(&winning, &ours);

    Ok((input, power))
}

fn parse_card_number(line: &str) -> IResult<&str, ()> {
    let (input, _) = delimited(
        tag("Card"),
        map_res(preceded(space1, digit1), |d: &str| d.parse::<u32>()),
        terminated(tag(":"), space1),
    )(line)?;

    Ok((input, ()))
}

fn parse_line(line: &str) -> IResult<&str, u32> {
    let (input, _) = parse_card_number(line)?;
    let (_, power) = parse_winning_numbers(input)?;

    Ok(("", power))
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32> {
    Ok(input.lines().map(|line| parse_line(line).unwrap().1).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 8)]
    #[case("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2)]
    #[case("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", 2)]
    #[case("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 1)]
    #[case("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 0)]
    #[case("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 0)]
    fn test_process(#[case] line: &str, #[case] expected: u32) -> miette::Result<()> {
        assert_eq!(expected, parse_line(line).unwrap().1);
        Ok(())
    }
}
