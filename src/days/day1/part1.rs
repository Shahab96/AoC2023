const INPUT: &'static str = include_str!("input");

fn process_line(line: &str) -> u32 {
    let chars = line
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    chars[0] * 10 + chars[chars.len() - 1]
}

pub fn run() -> u32 {
    INPUT.lines().map(process_line).sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("1abc2", 12)]
    #[case("pqr3stu8vwx", 38)]
    #[case("a1b2c3d4e5f", 15)]
    #[case("treb7uchet", 77)]
    fn test(#[case] line: &str, #[case] expected: u32) {
        assert_eq!(expected, process_line(line));
    }
}
