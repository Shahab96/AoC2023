use crate::custom_error::AocError;

use super::process_line;

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn process_words_in_line(line: &str) -> String {
    let mut buffer = String::new();
    let mut idx = 0;

    while idx < line.len() {
        let line_buf = &line[idx..];
        let digit = DIGITS.iter().position(|word| line_buf.starts_with(word));

        if let Some(digit) = digit {
            buffer.push_str(&format!("{}", digit + 1));
        } else {
            buffer.push(line.chars().nth(idx).expect("No char at index"));
        }

        idx += 1;
    }

    buffer
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    Ok(input
        .lines()
        .map(|line| process_line(&process_words_in_line(line)))
        .sum::<u32>())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    fn test_process(#[case] line: &str, #[case] expected: u32) {
        assert_eq!(expected, process_line(&process_words_in_line(line)));
    }
}
