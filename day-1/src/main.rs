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
    let mut num_1: Option<char> = None;
    let mut num_2: Option<char> = None;

    if text.len() < 1 {
        return None;
    }

    for character in text.chars() {
        if character.is_digit(10) & num_1.is_none(){
            num_1 = Some(character);
            break;
        }
    }

    for character in text.chars().rev() {
        if character.is_digit(10) & num_2.is_none(){
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