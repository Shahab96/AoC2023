use super::{process_line, Game};
use crate::custom_error::AocError;

fn filter_games(game: &Game) -> bool {
    game.rounds
        .iter()
        .all(|round| round.red <= 12 && round.green <= 13 && round.blue <= 14)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    Ok(input
        .lines()
        .map(process_line)
        .filter(filter_games)
        .map(|game| game.id)
        .sum::<u32>())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", true)]
    #[case(
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        true
    )]
    #[case(
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        false
    )]
    #[case(
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        false
    )]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", true)]
    fn test_process(#[case] line: &str, #[case] expected: bool) -> miette::Result<()> {
        assert_eq!(expected, filter_games(&process_line(line)));
        Ok(())
    }
}
