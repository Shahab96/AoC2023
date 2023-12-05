use std::collections::HashMap;

use crate::custom_error::AocError;

const SEARCH_COORDS: [(i32, i32); 8] = [
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0),
    (1, 1),
    (1, -1),
    (-1, -1),
    (-1, 1),
];

fn check_adjacent(
    matrix: &Vec<&str>,
    coords: (usize, usize),
    adjacent_gears: &mut Vec<(usize, usize)>,
) {
    for (x, y) in SEARCH_COORDS.iter() {
        let new_x = coords.0 as i32 + x;
        let new_y = coords.1 as i32 + y;

        // Check if we are out of the left bound.
        if new_y < 0 {
            continue;
        }

        // Check if we are out of the right bound.
        if new_y >= matrix[coords.0].len() as i32 {
            continue;
        }

        // Check if we are out of the top bound.
        if new_x < 0 {
            continue;
        }

        // Check if we are out of the bottom bound.
        if new_x >= matrix.len() as i32 {
            continue;
        }

        // Now we can use usizes
        let new_x = new_x as usize;
        let new_y = new_y as usize;

        // Check each of the 8 directions for a non-numeric symbol
        if let Some(element) = matrix[new_x].chars().nth(new_y) {
            if element == '*' {
                adjacent_gears.push((new_x, new_y));
            }
        }
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    let matrix = input.lines().collect::<Vec<&str>>();
    let mut hash_map: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    let mut line = matrix.get(x).unwrap();
    let mut chars = line.chars().enumerate();

    loop {
        if y == matrix[x].len() {
            x += 1;
            y = 0;

            if x == matrix.len() {
                break;
            }

            line = matrix.get(x).unwrap();
            chars = line.chars().enumerate();
            continue;
        }

        y = chars
            .find(|(_, c)| c.is_numeric())
            .map(|(idx, _)| idx)
            .unwrap_or(matrix[x].len());

        let end_of_num_idx = chars
            .find(|(_, c)| !c.is_numeric())
            .map(|(idx, _)| idx)
            .unwrap_or(matrix[x].len());

        let possible_valid_indices = y..end_of_num_idx;

        let mut adjacent_gears = vec![];

        for idx in possible_valid_indices {
            check_adjacent(&matrix, (x, idx), &mut adjacent_gears);
            if adjacent_gears.is_empty() {
                continue;
            }

            let number = &matrix[x][y..end_of_num_idx];
            let number = number.parse::<u32>().unwrap();

            adjacent_gears.iter().for_each(|coords| {
                hash_map.entry(*coords).or_default().push(number);
            });
        }

        y = end_of_num_idx;
    }

    // Remove duplicates from the vector
    hash_map.iter_mut().for_each(|(_, v)| {
        v.sort();
        v.dedup();
    });

    Ok(hash_map.values()
        .filter(|v| v.len() == 2)
        .map(|v| v.iter().product::<u32>())
        .sum::<u32>())
}
