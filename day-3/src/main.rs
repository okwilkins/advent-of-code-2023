use std::{usize, fs};
use regex::Regex;

fn main() {
    let file_path = parse_args();
    let text_contents = fs::read_to_string(file_path.input)
        .expect("Should have been able to read the file");

    parse_engine_schematic(&text_contents);
}

#[derive(Debug)]
struct Arguments {
    input: String,
}

fn parse_args() -> Arguments {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() != 1 {
        eprintln!("Wrong number of arguments: expected 1, got {}.", args.len());
    }

    Arguments {
        input: args[0].clone(),
    }
}

fn parse_engine_schematic(schematic: &str) {
    let num_re: Regex = Regex::new(r"[0-9]+").unwrap();
    let num_ranges: Vec<_> = num_re
                                .find_iter(schematic)
                                .map(|m| m.range())
                                .collect();
    let check_chars: [char; 10] = ['*', '#', '$', '/', '@', '%', '-', '+', '=', '&'];
    let mut schematic_sum: usize = 0;

    let test: Vec<&str> = num_ranges.iter().map(|num_range| schematic.get(num_range.start..num_range.end).unwrap()).collect();
    println!("Found {:?} numbers.", test.len());

    for num_range in num_ranges.iter() {
        for x in num_range.start..num_range.end {
            let surrounding_chars: [Option<char>; 8] = get_surrounding_chars(x,schematic);
            let num_is_adjacent_to_symbol = surrounding_chars.iter().flatten().any(|c| check_chars.contains(c));

            if num_is_adjacent_to_symbol {
                schematic_sum += schematic
                                    .get(num_range.start..num_range.end)
                                    .unwrap()
                                    .parse::<usize>()
                                    .unwrap();
                break;
            }
        }
    }

    println!("{}", schematic_sum);
}

/// Converts a 2D index to a 1D index.
/// 
/// # Arguments
/// x - The x coordinate of the cell.
/// y - The y coordinate of the cell.
/// row_len - The length of the row of the schematic.
/// 
/// # Returns
/// The 1D index of the cell.
fn index_2d_to_1d(x: usize, y: usize, row_len: usize) -> usize {
    (y * row_len) + x
}

/// Converts a 1D index to a 2D index.
/// 
/// # Arguments
/// i - The 1D index of the cell.
/// row_len - The length of the row.
/// 
/// # Returns
/// The 2D coordinates of the cell.
/// The coordinates are returned in the following order:
/// x, y
fn index_1d_to_2d(i: usize, row_len: usize) -> (usize, usize) {
    let y: usize = i / row_len;
    let x: usize = i % row_len;

    (x, y)
}

/// Calculates the coordinates of the surrounding cells.
/// If the cell is on the edge of the schematic, the cell will be ignored.
/// The cells are returned in the following order:
/// up, down, left, right, up right, up left, down right, down left
///
/// # Arguments
/// i - The 1D index of the cell.
/// text - The text of the schematic.
/// 
/// # Returns
/// The characters of the surrounding cells.
fn get_surrounding_chars(i: usize, text: &str) -> [Option<char>; 8] {
    // +1 to account for the newline character
    let row_len: usize = text.lines().nth(0).unwrap().len() + 1;

    // Get the 2d coordinates of the cell
    let coords: (usize, usize) = index_1d_to_2d(i, row_len);
    let x: usize = coords.0;
    let y: usize = coords.1;

    let x_left: Option<usize> = x.checked_sub(1);
    let x_right: Option<usize> = match x + 1 {
        x if x >= row_len => None,
        x => Some(x)
    };
    let y_up: Option<usize> = y.checked_sub(1);
    let y_down: Option<usize> = match y + 1 {
        y if y >= text.lines().count() => None,
        y => Some(y)
    };

    // Get the 1D index of the surrounding cells
    let up: Option<usize> = match y_up {
        None => None,
        Some(y) => Some(index_2d_to_1d(x, y, row_len)),
    };
    let down: Option<usize> = match y_down {
        None => None,
        Some(y) => Some(index_2d_to_1d(x, y, row_len)),
    };
    let left: Option<usize> = match x_left {
        None => None,
        Some(x) => Some(index_2d_to_1d(x, y, row_len)),
    };
    let right: Option<usize> = match x_right {
        None => None,
        Some(x) => Some(index_2d_to_1d(x, y, row_len)),
    };
    let up_right: Option<usize> = match (y_up, x_right) {
        (Some(y), Some(x)) => Some(index_2d_to_1d(x, y, row_len)),
        _ => None,
    };
    let up_left: Option<usize> = match (y_up, x_left) {
        (Some(y), Some(x)) => Some(index_2d_to_1d(x, y, row_len)),
        _ => None,
    };
    let down_right: Option<usize> = match (y_down, x_right) {
        (Some(y), Some(x)) => Some(index_2d_to_1d(x, y, row_len)),
        _ => None,
    };
    let down_left: Option<usize> = match (y_down, x_left) {
        (Some(y), Some(x)) => Some(index_2d_to_1d(x, y, row_len)),
        _ => None,
    };

    // Get the characters of the surrounding cells
    let up_char: Option<char> = match up {
        None => None,
        Some(i) => Some(text.chars().nth(i).unwrap()),
    };
    let down_char: Option<char> = match down {
        None => None,
        Some(i) => Some(text.chars().nth(i).unwrap()),
    };
    let left_char: Option<char> = match left {
        None => None,
        Some(i) => Some(text.chars().nth(i).unwrap()),
    };
    let right_char: Option<char> = match right {
        None => None,
        Some(i) => Some(text.chars().nth(i).unwrap()),
    };
    let up_right_char: Option<char> = match up_right {
        None => None,
        Some(i) => Some(text.chars().nth(i).unwrap()),
    };
    let up_left_char: Option<char> = match up_left {
        None => None,
        Some(i) => Some(text.chars().nth(i).unwrap()),
    };
    let down_right_char: Option<char> = match down_right {
        None => None,
        Some(i) => Some(text.chars().nth(i).unwrap()),
    };
    let down_left_char: Option<char> = match down_left {
        None => None,
        Some(i) => Some(text.chars().nth(i).unwrap()),
    };

    [up_char, down_char, left_char, right_char, up_right_char, up_left_char, down_right_char, down_left_char]
}