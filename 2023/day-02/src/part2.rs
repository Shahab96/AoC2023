use super::{process_line, Game};
use crate::custom_error::AocError;

impl Game {
    fn max(&self) -> (u32, u32, u32) {
        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;

        self.rounds.iter().for_each(|round| {
            red = std::cmp::max(red, round.red);
            blue = std::cmp::max(blue, round.blue);
            green = std::cmp::max(green, round.green);
        });

        (red, green, blue)
    }

    fn power(&self) -> u32 {
        let (red, green, blue) = self.max();
        red * green * blue
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    Ok(input
        .lines()
        .map(process_line)
        .map(|game| game.power())
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
        (4, 2, 6),
        48
    )]
    #[case(
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        (1, 3, 4),
        12
    )]
    #[case(
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        (20, 13, 6),
        1560
    )]
    #[case(
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        (14, 3, 15),
        630
    )]
    #[case(
        "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        (6, 3, 2),
        36
    )]
    fn test_process(
        #[case] line: &str,
        #[case] expected: (u32, u32, u32),
        #[case] power: u32,
    ) -> miette::Result<()> {
        assert_eq!(process_line(line).max(), expected);
        assert_eq!(process_line(line).power(), power);
        Ok(())
    }
}
