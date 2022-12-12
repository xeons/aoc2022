use std::collections::VecDeque;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str;

pub fn run() {
    let file = File::open("resources/day5input.txt").expect("Input file not found.");
    let reader = BufReader::new(file);

    let mut sorted_crates_part1: Vec<VecDeque<String>> = Vec::new();
    let mut sorted_crates_part2: Vec<VecDeque<String>> = Vec::new();
    let mut read_state: bool = true;

    for line in reader.lines() {
        let unwrapped_line = line.unwrap();
        if unwrapped_line.len() == 0 {
            sorted_crates_part1 = sorted_crates_part2.clone();
            read_state = false;
            continue;
        }

        if read_state {
            let chunks = unwrapped_line
                .as_bytes()
                .chunks(4)
                .map(str::from_utf8)
                .collect::<Result<Vec<&str>, _>>()
                .unwrap();

            for n in 0..chunks.len() {
                if chunks.len() > sorted_crates_part2.len() {
                    sorted_crates_part2.push(VecDeque::new());
                }
                if chunks[n].contains("[") {
                    sorted_crates_part2[n].push_back(chunks[n].to_string());
                }
            }
        } else {
            if unwrapped_line.contains("move") {
                let args = unwrapped_line.split(" ").collect::<Vec<&str>>();
                let num_crates_move = args[1].parse::<usize>().unwrap();
                let source_col = args[3].parse::<usize>().unwrap();
                let dest_col = args[5].parse::<usize>().unwrap();

                for _ in 0..num_crates_move {
                     let current_crate = sorted_crates_part1[source_col - 1].pop_front().unwrap();
                     sorted_crates_part1[dest_col - 1].push_front(current_crate);
                }

                let mut crates: Vec<String> = sorted_crates_part2[source_col - 1]
                    .drain(0..num_crates_move)
                    .collect();
                crates.reverse();
                for c in crates {
                    sorted_crates_part2[dest_col - 1].push_front(c);
                }
            }
        }
    }

    let answer1:Vec<String> = sorted_crates_part1
        .iter()
        .map(|x| x.clone().pop_front().unwrap().trim().replace("[","").replace("]",""))
        .collect();
    println!("Day 5 Answer 1: {:?}", answer1.join(""));


    let answer2:Vec<String> = sorted_crates_part2
        .iter()
        .map(|x| x.clone().pop_front().unwrap().trim().replace("[","").replace("]",""))
        .collect();
    println!("Day 5 Answer 2: {:?}", answer2.join(""));
}