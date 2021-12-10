// mod input_loader;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;

use std::time::Instant;

fn main() {
    let start = Instant::now();

    let args: Vec<_> = std::env::args().collect();

    let day = match args.get(1) {
        Some(day) => day,
        None => panic!("Please provide a day"),
    };

    match day.as_str() {
        "day_1" => day_01::run(),
        "day_2" => day_02::run(),
        "day_3" => day_03::run(),
        "day_4" => day_04::run(),
        "day_5" => day_05::run(),
        "day_6" => day_06::run(),
        "day_7" => day_07::run(),
        "day_8" => day_08::run(),
        "day_9" => day_09::run(),
        "day_10" => day_10::run(),
        "all" => {
            day_01::run();
            day_02::run();
            day_03::run();
            day_04::run();
            day_05::run();
            day_06::run();
            day_07::run();
            day_08::run();
            day_09::run();
            day_10::run();
        }
        _ => panic!("Provided day does not exists"),
    };

    println!("Program finished in {} seconds", start.elapsed().as_secs());
}
