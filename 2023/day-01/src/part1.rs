use crate::custom_error::AocError;

use super::process_line;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    Ok(input.lines().map(process_line).sum::<u32>())
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
    fn test_process(#[case] line: &str, #[case] expected: u32) {
        assert_eq!(expected, process(line).expect("Failed to process line"));
    }
}
