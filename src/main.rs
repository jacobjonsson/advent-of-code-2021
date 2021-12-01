use std::time::Instant;

mod day_1;
mod input_loader;

fn main() {
    let start = Instant::now();

    let args: Vec<_> = std::env::args().collect();

    let day = match args.get(1) {
        Some(day) => day,
        None => panic!("Please provide a day"),
    };

    match day.as_str() {
        "day_1" => day_1::run(),

        _ => panic!("Provided day does not exists"),
    };

    println!("Program finished in {} seconds", start.elapsed().as_secs());
}
