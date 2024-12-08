use clap::{Parser, ValueHint};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    // TODO: Add a value_parser for timers so that only positive values are valid
    /// Duration of work sessions (minutes)
    #[arg(short, long, value_name = "MINS", default_value_t = 25)]
    pub work: i32,

    /// Duration of long breaks (minutes)
    #[arg(short, long, value_name = "MINS", default_value_t = 15)]
    pub long_break: i32,

    /// Duration of short breaks (minutes)
    #[arg(short, long, value_name = "MINS", default_value_t = 5)]
    pub short_break: i32,

    /// Audio file to play when a session ends
    #[arg(short, long, value_name = "FILE", value_hint = ValueHint::FilePath)]
    pub alarm: Option<PathBuf>,

    /// Send a notification when a session ends
    #[arg(short, long)]
    pub notify: bool,

    /// Automatically start sessions
    #[arg(short, long)]
    pub quick_start: bool,
    /*
    TODO: Figure out how to make alarm_repeats work with quick_start elegantly.

    Maybe play the alarm sound *once* and then auto start the next session when quick_start is enabled,
    and if it's disabled, just loop the alarm until the user interacts with the UI to start the next session?
    :p

    /// Number of times to repeat the alarm
    #[arg(short, long)]
    pub alarm_repeats: Option<i32>,
    */
}
