use clap::{value_parser, Parser, ValueHint};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about)]
pub struct Args {
    /// Work session duration
    #[arg(short, long, default_value_t = 25, value_name = "MINS", value_parser = value_parser!(u32).range(1..))]
    pub work: u32,

    /// Long break duration
    #[arg(short, long, default_value_t = 15, value_name = "MINS", value_parser = value_parser!(u32).range(1..))]
    pub long_break: u32,

    /// Short break duration
    #[arg(short, long, default_value_t = 5, value_name = "MINS", value_parser = value_parser!(u32).range(1..))]
    pub short_break: u32,

    /// Sound file to play when a session is finished
    #[arg(short, long, value_name = "FILE", value_hint = ValueHint::FilePath)]
    pub alarm: Option<PathBuf>,

    /// Send a notification when a session is finished
    #[arg(short, long)]
    pub notify: bool,
}
