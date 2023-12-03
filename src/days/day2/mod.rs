pub mod part1;
pub mod part2;

pub const INPUT: &str = include_str!("input.txt");

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
