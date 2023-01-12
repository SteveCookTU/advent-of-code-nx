use alloc::string::String;

mod day1;

pub fn run_day(day: i32) -> String {
    match day {
        1 => day1::run(),
        _ => String::from("Day not yet implemented!"),
    }
}
