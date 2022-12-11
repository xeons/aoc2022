use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
struct ElfAssignment {
    first_range_start: u8,
    first_range_end: u8,
    second_range_start: u8,
    second_range_end: u8,
}

pub fn run() {
    let file = File::open("resources/day4input.txt").expect("Input file not found.");
    let reader = BufReader::new(file);

    let re = Regex::new(r"^(\d{1,2})-(\d{1,2}),(\d{1,2})-(\d{1,2})$").unwrap();
    let assignments: Vec<Option<ElfAssignment>> = reader
        .lines()
        .map(|line| {
            let unwrapped_line = line.unwrap();
            let cap = re.captures(unwrapped_line.as_str()).unwrap();
            let groups = (cap.get(1), cap.get(2), cap.get(3), cap.get(4));
            match groups {
                (Some(r1), Some(r2), Some(r3), Some(r4)) => Some(ElfAssignment {
                    first_range_start: r1.as_str().parse::<u8>().unwrap(),
                    first_range_end: r2.as_str().parse::<u8>().unwrap(),
                    second_range_start: r3.as_str().parse::<u8>().unwrap(),
                    second_range_end: r4.as_str().parse::<u8>().unwrap(),
                }),
                _ => None,
            }
        })
        .collect();

    let mut total_overlap: u32 = 0;
    let mut partial_overlap: u32 = 0;

    for assignment in assignments {
        let a = assignment.unwrap();
        if (a.first_range_start >= a.second_range_start
            && a.first_range_end <= a.second_range_end) ||
            (a.second_range_start >= a.first_range_start
                && a.second_range_end <= a.first_range_end) {
            total_overlap += 1;
        }

        if ((a.first_range_start >= a.second_range_start && a.first_range_start <= a.second_range_end) ||
            (a.first_range_end >= a.second_range_start && a.first_range_end <= a.second_range_end)) ||
            ((a.second_range_start >= a.first_range_start && a.second_range_start <= a.first_range_end) ||
                (a.second_range_end >= a.first_range_start && a.second_range_end <= a.first_range_end)) {
            partial_overlap += 1;
        }
    }

    println!("Day 4 Answer 1: {:?}", total_overlap);
    println!("Day 4 Answer 2: {:?}", partial_overlap);
}