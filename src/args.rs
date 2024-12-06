use clap::{Parser, ValueEnum};

use crate::days::Task;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    pub day: String,
    pub task: ArgTask,
    #[arg(short, long, help = "Enable tracking the time a task needs finish")]
    pub time: bool,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum ArgTask {
    First,
    Second,
}

impl From<ArgTask> for Task {
    fn from(val: ArgTask) -> Self {
        match val {
            ArgTask::First => Task::First,
            ArgTask::Second => Task::Second,
        }
    }
}
