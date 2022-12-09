use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub fn run() {
    let file = File::open("resources/day1input.txt").expect("Input file not found.");
    let reader = BufReader::new(file);
    let mut calorie_totals: Vec<i64> = Vec::new();
    let mut current_elf_calories = 0;

    for line in reader.lines() {
        let calories = line.unwrap().parse::<i64>().unwrap_or(0);
        println!("Input calories: {:?}", calories);
        if calories == 0 {
            println!("Adding to current elf total: {:?}", current_elf_calories);
            calorie_totals.push(current_elf_calories);
            current_elf_calories = 0;
        } else {
            current_elf_calories += calories;
        }
    }

    if current_elf_calories > 0 {
        println!("Adding to current elf total: {:?}", current_elf_calories);
        calorie_totals.push(current_elf_calories);
    }

    calorie_totals.sort();
    calorie_totals.reverse();

    assert!(calorie_totals.len() >= 3);
    println!("Day 1 Part A Answer: {}", calorie_totals[0]);
    println!("Day 1 Part B Answer: {}", calorie_totals[0] + calorie_totals[1] + calorie_totals[2]);
}
