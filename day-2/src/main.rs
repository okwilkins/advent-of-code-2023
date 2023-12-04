use std::{usize, fs};
use regex::Regex;

fn main() {
    let file_path = parse_args();
    let text_contents = fs::read_to_string(file_path.input)
        .expect("Should have been able to read the file");
    let check_bag = Bag::new(12, 13, 14);
    let mut sum_of_game_ids: usize = 0;
    let mut sum_of_game_powers: usize = 0;
    

    for line in text_contents.lines() {
        println!("Line: {}", line);
        println!("Parsed: {:?}", parse_game(line));

        let (game_id, parsed_bag): (usize, Bag) = parse_game(line)
            .expect("Should have been able to parse the game!");

        let valid_game: bool = parsed_bag.red_cubes <= check_bag.red_cubes
            && parsed_bag.green_cubes <= check_bag.green_cubes
            && parsed_bag.blue_cubes <= check_bag.blue_cubes;
        
        if valid_game {
            sum_of_game_ids += game_id;
        }

        let game_power: usize = parsed_bag.red_cubes * parsed_bag.green_cubes * parsed_bag.blue_cubes;
        sum_of_game_powers += game_power;

        println!("Game ID: {}", game_id);
        println!("Bag: {:?}", parsed_bag);
        println!("Valid game: {}", valid_game);
        println!("Game power: {}", game_power);
        println!("");
    }

    println!("Sum of game IDs: {}", sum_of_game_ids);
    println!("Sum of game powers: {}", sum_of_game_powers);
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

#[derive(Debug)]
struct Bag {
    red_cubes: usize,
    green_cubes: usize,
    blue_cubes: usize,
}

impl Bag {
    fn new(red_cubes: usize, green_cubes: usize, blue_cubes: usize) -> Self {
        Bag {
            red_cubes,
            green_cubes,
            blue_cubes,
        }
    }

    fn update_cubes(&mut self, red_cubes: usize, green_cubes: usize, blue_cubes: usize) {
        if red_cubes > self.red_cubes {
            self.red_cubes = red_cubes;
        }

        if green_cubes > self.green_cubes {
            self.green_cubes = green_cubes;
        }

        if blue_cubes > self.blue_cubes {
            self.blue_cubes = blue_cubes;
        }
    }
}

fn parse_game(text: &str) -> Option<(usize, Bag)> {
    let re = Regex::new(r"Game\s([0-9]+):\s(.+)$").unwrap();
    let mut bag = Bag::new(0, 0, 0);

    let captures = re.captures(text).expect("Should have been able to get captures!");
    let game_id_match = captures
            .get(1)
            .expect("Should have been able to get a game ID from the capture!");
    let pulls_cap = captures
        .get(2)
        .expect("Should have been able to get a bag pull from the capture!");

    let game_id: usize = game_id_match
        .as_str()
        .parse()
        .expect("Should have been able to parse into a usize!");


    for pull in pulls_cap.as_str().split(";") {
        let cubes: (usize, usize, usize) = parse_cubes(pull);
        bag.update_cubes(cubes.0, cubes.1, cubes.2);
    }

    return Some((game_id, bag));
}


#[derive(Debug)]
enum Colour {
    Red,
    Green,
    Blue,
}

fn parse_cubes(text: &str) -> (usize, usize, usize) {
    let mut red_cubes: Vec<usize> = Vec::new();
    let mut green_cubes: Vec<usize> = Vec::new();
    let mut blue_cubes: Vec<usize> = Vec::new();

    let re = Regex::new(r"([0-9]+)\s(red|green|blue)").unwrap();
    
    for capture in re.captures_iter(text) {
        let num_cubes: usize = capture
            .get(1)
            .expect("Should have been able to get the number of cubes!")
            .as_str()
            .parse::<usize>()
            .expect("Should have been able to parse into a usize!");

        let colour: Colour = capture.get(2).map(
            |m| match m.as_str() {
                "red" => Colour::Red,
                "green" => Colour::Green,
                "blue" => Colour::Blue,
                _ => Colour::Red,
            }
        ).expect("Should have been able to parse into colour!");

        match colour {
            Colour::Red => { red_cubes.push(num_cubes); },
            Colour::Green => { green_cubes.push(num_cubes); },
            Colour::Blue => { blue_cubes.push(num_cubes); },
        }
    }

    let mut red_cube_sum: usize = 0;
    let mut green_cube_sum: usize = 0;
    let mut blue_cube_sum: usize = 0;

    for num_red_cubes in red_cubes {
        red_cube_sum += num_red_cubes;
    }

    for num_green_cubes in green_cubes {
        green_cube_sum += num_green_cubes;
    }
    
    for num_blue_cubes in blue_cubes {
        blue_cube_sum += num_blue_cubes;
    }

    return (red_cube_sum, green_cube_sum, blue_cube_sum);
}