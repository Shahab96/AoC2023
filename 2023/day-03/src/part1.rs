use crate::custom_error::AocError;

#[derive(Debug)]
struct Coordinates {
    data: u32,
    x: usize,
    y: usize,
}

fn find_symbol<'a>(
    line: &str,
    line_num: usize,
    prev: Option<&&str>,
    coords: &'a mut Vec<Coordinates>,
) -> &'a mut Vec<Coordinates> {
    let mut x = 0;

    let symbol_idx = line.find(|c: char| c.is_ascii_punctuation());

    if symbol_idx.is_none() {
        return coords;
    }

    // TODO:
    // If you find a symbol, get the numbers in front and behind it.
    //
    // Generate coordinates for each number.
    //
    // Repeat for the previous line, if the number at the appropriate coordinates is not already in the list.

    todo!()
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut coords: Vec<Coordinates> = vec![];
    let lines = input.lines().collect::<Vec<&str>>();

    for (y, line) in lines.iter().enumerate() {
        find_symbol(line, y, lines.get(y - 1), &mut coords);
    }
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        todo!("haven't built test yet");
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
