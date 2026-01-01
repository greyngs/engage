use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "engage",
    about = "A simple terminal-based time management tool.",
    version = "0.1.0"
)]
pub struct Cli {
    /// Mute audible alerts
    #[arg(long, short = 'm', global = true)]
    pub mute: bool,

    /// Use ASCII font (Figlet) instead of Block characters
    #[arg(long, global = true)]
    pub ascii: bool,

    /// Use simple console output (no TUI)
    #[arg(long, global = true)]
    pub simple: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start a countdown timer
    Timer {
        /// Duration sequence (e.g., "1h 10m 30s")
        #[arg(required = true, help = "Input duration (e.g., '10m 30s')")]
        durations: Vec<String>,
    },
    /// Start a stopwatch
    Chrono,
    /// Display the current time
    Clock {
        /// Show seconds in the clock
        #[arg(short, long)]
        seconds: bool,
    },
    /// Start a Pomodoro session
    Pomodoro {
        /// Focus duration in minutes
        #[arg(short, long, default_value_t = 25)]
        focus: u64,

        /// Short break duration in minutes
        #[arg(short, long, default_value_t = 5)]
        short: u64,

        /// Long break duration in minutes
        #[arg(short, long, default_value_t = 15)]
        long: u64,
    },
}
