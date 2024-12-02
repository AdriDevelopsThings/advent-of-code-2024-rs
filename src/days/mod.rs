use std::env;

mod day_1;
mod day_2;

type DayFunction = fn(String) -> String;
const DAYS: &[(&str, DayFunction, DayFunction)] = &[
    ("1", day_1::first, day_1::second),
    ("2", day_2::first, day_2::second),
];

pub enum Task {
    First,
    Second,
}

pub fn solve_day(day: String, task: Task) -> String {
    let session = env::var("ADVENT_OF_CODE_SESSION")
        .expect("Environment variable 'ADVENT_OF_CODE_SESSION' was not set.");

    for d in DAYS {
        if d.0 == day {
            let function = match task {
                Task::First => d.1,
                Task::Second => d.2,
            };
            let response = reqwest::blocking::Client::new()
                .get(format!("https://adventofcode.com/2024/day/{day}/input"))
                .header("Cookie", format!("session={session}"))
                .send()
                .unwrap()
                .error_for_status()
                .unwrap();
            let input = response.text().unwrap();
            let output = function(input);
            return output;
        }
    }
    panic!("Day {day} was not found.");
}
