use super::INPUT;

#[derive(Debug, Default, Eq, PartialEq)]
struct Round {
    red: u32,
    blue: u32,
    green: u32,
}

#[derive(Debug, Default, Eq, PartialEq)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

fn process_line(line: &str) -> Game {
    let split = line.find(':').unwrap();
    let id = &line[..split]
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let mut game = Game {
        id: *id,
        ..Default::default()
    };

    line[split + 1..].split_terminator(';').for_each(|round| {
        let round_data = round
            .split_whitespace()
            .map(|x| x.trim_end_matches(','))
            .collect::<Vec<_>>()
            .into_iter();

        round_data
            .clone()
            .step_by(2)
            .zip(round_data.clone().skip(1).step_by(2))
            .for_each(|(count, color)| {
                let count = count.parse::<u32>().unwrap();
                let mut round = Round::default();
                match color {
                    "red" => round.red += count,
                    "blue" => round.blue += count,
                    "green" => round.green += count,
                    _ => unreachable!(),
                };

                game.rounds.push(round);
            });
    });

    game
}

fn filter_games(game: &Game) -> bool {
    game.rounds
        .iter()
        .all(|round| round.red <= 12 && round.green <= 13 && round.blue <= 14)
}

pub fn run() -> u32 {
    INPUT
        .lines()
        .map(process_line)
        .filter(filter_games)
        .map(|game| game.id)
        .sum::<u32>()
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
    fn day2_part1(#[case] line: &str, #[case] expected: bool) {
        assert_eq!(expected, filter_games(&process_line(line)));
    }
}
