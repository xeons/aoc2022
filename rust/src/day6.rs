use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn run() {
    let file = File::open("resources/day6input.txt").expect("Input file not found.");
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    let _ = reader.read_line(&mut line);

    let chars: Vec<_> = line.chars().collect();

    const START_OF_PACKET_SIZE: usize = 4;
    const START_OF_MESSAGE_SIZE: usize = 14;

    let start_of_packet: usize = find_unique_fragment_position(START_OF_PACKET_SIZE, &chars);
    let start_of_message: usize = find_unique_fragment_position(START_OF_MESSAGE_SIZE, &chars);

    println!("Day 6 Answer 1: {:?}", start_of_packet);
    println!("Day 6 Answer 2: {:?}", start_of_message);
}

fn find_unique_fragment_position(fragment_size: usize, chars: &Vec<char>) -> usize {
    let mut fragment_position: usize = 0;
    for i in fragment_size..chars.len() {
        let mut no_match: bool = true;
        for j in 0..fragment_size {
            for k in 0..fragment_size {
                if j != k && chars[i - j] == chars[i - k] {
                    no_match = false;
                }
            }
        }
        if no_match {
            fragment_position = i + 1;
            break;
        }
    }
    fragment_position
}