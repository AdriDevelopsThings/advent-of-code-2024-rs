use args::Args;
use clap::Parser;
use days::solve_day;

mod args;
mod days;

fn main() {
    dotenv::dotenv().ok();
    let args = Args::parse();
    let output = solve_day(args.day, args.task.into());
    println!("Output:\n{output}");
}
