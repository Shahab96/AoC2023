use std::collections::BTreeMap;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space0, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
};

fn get_num_matches(winning: &[u32], ours: &[u32]) -> u32 {
    let mut matches = 0;

    for &number in ours {
        if winning.contains(&number) {
            matches += 1;
        }
    }

    matches
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

    let matches = get_num_matches(&winning, &ours);

    Ok((input, matches))
}

fn parse_card_number(line: &str) -> IResult<&str, u32> {
    let (input, card_number) = delimited(
        tag("Card"),
        map_res(preceded(space1, digit1), |d: &str| d.parse::<u32>()),
        terminated(tag(":"), space1),
    )(line)?;

    Ok((input, card_number))
}

fn parse_line(line: &str) -> IResult<&str, (u32, u32)> {
    let (input, card_number) = parse_card_number(line)?;
    let (_, matches) = parse_winning_numbers(input)?;

    Ok(("", (card_number, matches)))
}

// fn calculate_next_match_set(card_number: u32, num_matches: u32, cards: &HashMap<u32, u32>) -> u32 {
//     if num_matches == 0 {
//         return 0;
//     }
//
//     let mut total = num_matches;
//     let range = (card_number + 1)..=(card_number + num_matches);
//
//     for i in range {
//         let next_card = cards.get(&i).unwrap();
//         total += calculate_next_match_set(i, *next_card, cards);
//     }
//
//     total
// }

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32> {
    let cards = input
        .lines()
        .map(|line| parse_line(line).unwrap().1 .1)
        .collect::<Vec<_>>();

    let tree = (0..cards.len())
        .map(|idx| (idx, 1))
        .collect::<BTreeMap<usize, u32>>();

    let total = cards
        .iter()
        .enumerate()
        .fold(tree, |mut acc, (idx, num_matches)| {
            for i in (idx + 1)..(idx + 1 + *num_matches as usize) {
                let copies = *acc.get(&i).unwrap();
                acc.entry(i).and_modify(|e| *e += copies);
            }

            acc
        })
        .values()
        .sum::<u32>();

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", (1, 4))]
    #[case("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", (2, 2))]
    #[case("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", (3, 2))]
    #[case("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", (4, 1))]
    #[case("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", (5, 0))]
    #[case("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", (6, 0))]
    fn test_process(#[case] line: &str, #[case] expected: (u32, u32)) -> miette::Result<()> {
        assert_eq!(expected, parse_line(line).unwrap().1);
        Ok(())
    }
}
