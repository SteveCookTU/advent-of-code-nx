use alloc::string::String;

mod day1;

pub fn run_day(day: i32, input: &str) -> String {
    match day {
        1 => day1::run(input),
        _ => String::from("Day not yet implemented!"),
    }
}
