use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn run() {
    let file = File::open("resources/day2input.txt").expect("Input file not found.");
    let reader = BufReader::new(file);

    let mut part1_score: u32 = 0;
    let mut part2_score: u32 = 0;

    for line in reader.lines() {
        let str = line.unwrap();
        part1_score += calculate_score_part1(&str);
        part2_score += calculate_score_part2(&str);
    }

    println!("Day 2 Part 1 Answer: {:?}", part1_score);
    println!("Day 2 Part 2 Answer: {:?}", part2_score);
}

fn calculate_score_part1(game: &str) -> u32 {
    return match &game[..] {
        "A X" => 3 + 1,
        "A Y" => 6 + 2,
        "A Z" => 0 + 3,
        "B X" => 0 + 1,
        "B Y" => 3 + 2,
        "B Z" => 6 + 3,
        "C X" => 6 + 1,
        "C Y" => 0 + 2,
        "C Z" => 3 + 3,
        _ => panic!()
    };
}

fn calculate_score_part2(game: &str) -> u32 {
    return match &game[..] {
        "A X" => 3 + 0, // lose
        "A Y" => 1 + 3, // draw
        "A Z" => 2 + 6, // win
        "B X" => 1 + 0, // lose
        "B Y" => 2 + 3, // draw
        "B Z" => 3 + 6, // win
        "C X" => 2 + 0, // lose
        "C Y" => 3 + 3, // draw
        "C Z" => 1 + 6, // win
        _ => panic!()
    };
}