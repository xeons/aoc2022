use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn run() {
    let file = File::open("resources/day3input.txt").expect("Input file not found.");
    let reader = BufReader::new(file);

    let mut priority_sum1 = 0;
    let mut priority_sum2 = 0;
    let mut counter: usize = 0;
    let mut group_sacks: Vec<String> = vec![String::new(), String::new(), String::new()];

    for line in reader.lines() {
        // Part 1
        let current_line = line.unwrap();
        let (compartment1, compartment2) = current_line.split_at(current_line.len() / 2);
        let common_item_code = get_common_item_code(compartment1, compartment2);
        priority_sum1 += get_priority(common_item_code);

        // Part 2
        counter += 1;
        group_sacks[counter % 3] = current_line;
        if counter % 3 == 0 {
            let badge_item_code = get_common_item_code_part2(&group_sacks[0][..], &group_sacks[1][..], &group_sacks[2][..]);
            priority_sum2 += get_priority(badge_item_code);
        }
    }

    println!("Day 3 Answer 1: {:?}", priority_sum1);
    println!("Day 3 Answer 2: {:?}", priority_sum2);
}

fn get_priority(item: char) -> usize {
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".find(item).unwrap() + 1
}

fn get_common_item_code(c1: &str, c2: &str) -> char {
    c1.chars().find(|&c| c2.contains(c)).unwrap()
}

fn get_common_item_code_part2(c1: &str, c2: &str, c3: &str) -> char {
    c1.chars().find(|&c| c1.contains(c) && c2.contains(c) && c3.contains(c)).unwrap()
}