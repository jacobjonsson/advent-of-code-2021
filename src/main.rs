mod input_loader;

mod day_1;
mod day_2;
mod day_3;

use std::time::Instant;

fn main() {
    let start = Instant::now();

    let args: Vec<_> = std::env::args().collect();

    let day = match args.get(1) {
        Some(day) => day,
        None => panic!("Please provide a day"),
    };

    match day.as_str() {
        "day_1" => day_1::run(),
        "day_2" => day_2::run(),
        "day_3" => day_3::run(),
        "all" => {
            day_1::run();
            day_2::run();
            day_3::run();
        }
        _ => panic!("Provided day does not exists"),
    };

    println!("Program finished in {} seconds", start.elapsed().as_secs());
}
