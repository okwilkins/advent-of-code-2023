use std::{usize, fs};

fn main() {
    let file_path = parse_args();
    let text_contents = fs::read_to_string(file_path.input)
        .expect("Should have been able to read the file");
    let mut sum: usize = 0;

    for line in text_contents.lines() {
        match get_num_from_text(line) {
            Some(num) => {
                sum += num;
                println!("Found number: {}", num);
            },
            None => continue,
        }
        println!("");
    }

    println!("Sum: {}", sum);
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

fn get_num_from_text(text: &str) -> Option<usize> {
    println!("Original text: {}", text);
    if text.len() < 1 {
        return None;
    }

    let mut num_1: Option<char> = None;
    let mut num_2: Option<char> = None;
    let modified_text: String = convert_string_nums_to_num(text);
    println!("Modified text: {}", modified_text);

    for character in modified_text.chars() {
        if character.is_digit(10) {
            num_1 = Some(character);
            break;
        }
    }

    for character in modified_text.chars().rev() {
        if character.is_digit(10) {
            num_2 = Some(character);
            break;
        }
    }

    match (num_1, num_2) {
        (Some(num_1), Some(num_2)) => {
            return Some(
                (num_1.to_string() + &num_2.to_string())
                    .parse::<usize>().unwrap()
            )
        },
        _ => return None,
    }
}

/// With a given piece of text, convert all text representations of numbers to numbers.
/// If there is overlap between the numbers, only the first number will be converted.
/// 
/// # Arguments
/// * `text` - The text to search for numbers in.
/// 
/// # Returns
/// The modified text.
fn convert_string_nums_to_num(text: &str) -> String {
    let mut index = 0;

    let text_iter = std::iter::from_fn(move || {
        let reduced_text = &text[index..];

        let result = if reduced_text.starts_with("one") {
            Some('1')
        } else if reduced_text.starts_with("two") {
            Some('2')
        } else if reduced_text.starts_with("three") {
            Some('3')
        } else if reduced_text.starts_with("four") {
            Some('4')
        } else if reduced_text.starts_with("five") {
            Some('5')
        } else if reduced_text.starts_with("six") {
            Some('6')
        } else if reduced_text.starts_with("seven") {
            Some('7')
        } else if reduced_text.starts_with("eight") {
            Some('8')
        } else if reduced_text.starts_with("nine") {
            Some('9')
        } else {
            let result = reduced_text.chars().next();
            result
        };
        index += 1;
        result
    });

    let result: String = text_iter.into_iter().collect();
    return result
}
