use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub fn run() {
    let file = File::open("resources/day2input.txt").expect("Input file not found.");
    let reader = BufReader::new(file);

    let mut part1_score: u32 = 0;
    let mut part2_score: u32 = 0;

    for line in reader.lines() {
        let str = line.unwrap();
        let input_arguments = str.split(" ").collect::<Vec<&str>>();

        println!("Round: {:?}, {:?}", input_arguments[0], input_arguments[1]);

        let instruction_a = input_arguments[0].chars().next().unwrap();
        let instruction_b = input_arguments[1].chars().next().unwrap();

        if instruction_b == 'X' { 
            part1_score += check_outcome(instruction_b, instruction_a) + get_shape_value(instruction_b); // rock
            part2_score += get_shape_value(get_losing_move(instruction_a)); // lose
        } else if instruction_b == 'Y' { 
            part1_score += check_outcome(instruction_b, instruction_a) + get_shape_value(instruction_b); // paper
            part2_score += 3 + get_shape_value(instruction_a); // draw
        } else if instruction_b == 'Z' { 
            part1_score += check_outcome(instruction_b, instruction_a) + get_shape_value(instruction_b); // scissors
            part2_score += 6 + get_shape_value(get_winning_move(instruction_a)); // win
        }
    }

    println!("Day 2 Part 1 Answer: {:?}", part1_score);
    println!("Day 2 Part 2 Answer: {:?}", part2_score);
}

fn get_shape_value(shape: char) -> u32 {
    return match shape {
        'X' | 'A' => 1, // rock
        'Y' | 'B' => 2, // paper
        'Z' | 'C' => 3, // scissor
        _ => panic!()
    }
}

fn get_losing_move(shape: char) -> char {
    return match shape {
        'X' | 'A' => 'C', // rock
        'Y' | 'B' => 'A', // paper
        'Z' | 'C' => 'B', // scissor
        _ => panic!()
    } 
}

fn get_winning_move(shape: char) -> char {
    return match shape {
        'X' | 'A' => 'B', // rock
        'Y' | 'B' => 'C', // paper
        'Z' | 'C' => 'A', // scissor
        _ => panic!()
    } 
}

fn check_outcome(a: char, b: char) -> u32 {
    if get_shape_value(a) == get_shape_value(b) {
        return 3;
    } else if get_losing_move(a) == b {
        return 6;
    }
    return 0;
}