pub mod custom_error;

pub mod part1;
pub mod part2;

fn process_line(line: &str) -> u32 {
    let chars = line
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    chars[0] * 10 + chars[chars.len() - 1]
}
