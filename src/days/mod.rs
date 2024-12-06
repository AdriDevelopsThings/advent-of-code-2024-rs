use std::{env, time::Instant};

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;

type DayFunction = fn(String) -> String;
const DAYS: &[(&str, DayFunction, DayFunction)] = &[
    ("1", day_1::first, day_1::second),
    ("2", day_2::first, day_2::second),
    ("3", day_3::first, day_3::second),
    ("4", day_4::first, day_4::second),
    ("5", day_5::first, day_5::second),
    ("6", day_6::first, day_6::second),
];

pub enum Task {
    First,
    Second,
}

pub fn solve_day(day: String, task: Task, time: bool) -> String {
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
            let start = Instant::now();
            let output = function(input);
            let end = Instant::now();
            let duration = end - start;
            if time {
                println!("Task took {}ms.", duration.as_millis());
            }
            return output;
        }
    }
    panic!("Day {day} was not found.");
}
