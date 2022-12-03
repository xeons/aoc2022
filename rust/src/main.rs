use std::env;

mod day1;
mod day2;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            let day_to_run = &args[1];
            match &day_to_run[..] {
                "day1" => day1::run(),
                "day2" => day2::run(),
                _ => eprintln!("error: invalid day")
            }
        },
        _ => eprintln!("Error: not enough or too many arguments.")
    };
}
