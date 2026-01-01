mod app;
mod cli;
mod utils;
mod tui;

use app::{App, AppMode, PomodoroConfig, PomodoroPhase};
use clap::Parser;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::{
    io::{self, Write},
    time::{Duration, Instant},
};

fn main() -> io::Result<()> {
    let args = cli::Cli::parse();

    // Initialize App State based on CLI command
    let initial_mode = match args.command {
        cli::Commands::Chrono => AppMode::Chrono,
        cli::Commands::Timer { durations } => {
            let seconds = utils::parse_duration_from_args(&durations);
            AppMode::Timer { target: seconds }
        }
        cli::Commands::Pomodoro { focus, short, long } => {
            // Only print kickoff config in simple mode or before TUI init
            if args.simple {
                println!(
                    "Config: Focus {}m, Short Break {}m, Long Break {}m",
                    focus, short, long
                );
            }
            // Small delay only for simple mode to read config if desired, 
            // but for TUI it's better to start immediately.
            if args.simple {
                std::thread::sleep(Duration::from_secs(1));
            }

            AppMode::Pomodoro {
                phase: PomodoroPhase::Focus,
                cycles: 0,
                config: PomodoroConfig {
                    focus: focus * 60,
                    short: short * 60,
                    long: long * 60,
                },
            }
        }
        cli::Commands::Clock { seconds } => AppMode::Clock {
            show_seconds: seconds,
        },
    };

    let mut app = App::new(initial_mode, args.mute);

    if args.simple {
        // Run Original Simple Mode
        enable_raw_mode()?;
        if let Err(e) = run_simple_loop(&mut app) {
            disable_raw_mode()?;
            eprintln!("Error: {}", e);
            return Err(e);
        }
        disable_raw_mode()?;
    } else {
        // Run Ratatui Mode (Block or ASCII)
        // Raw mode is handled inside run_tui or wrapping it
        enable_raw_mode()?;
        if let Err(e) = tui::run_tui(&mut app, args.ascii) {
            disable_raw_mode()?;
            eprintln!("Error: {}", e);
            return Err(e);
        }
        disable_raw_mode()?;
    }

    Ok(())
}

// Renamed from run_loop to run_simple_loop
fn run_simple_loop(app: &mut App) -> io::Result<()> {
    let mut time_accumulated = Duration::ZERO;
    let mut last_tick = Instant::now();
    let mut current_target = app.get_target_duration();

    loop {
        // Input Handling
        if event::poll(Duration::from_millis(50))? 
            && let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char(' ') => {
                        app.paused = !app.paused;
                        if !app.paused {
                            last_tick = Instant::now();
                        }
                    }
                    KeyCode::Char('r') => {
                        time_accumulated = Duration::ZERO;
                        last_tick = Instant::now();
                    }
                    _ => {}
                }
            }

        // Time Logic
        if !app.paused {
            let now = Instant::now();
            time_accumulated += now.duration_since(last_tick);
            last_tick = now;
        }

        let elapsed_secs = time_accumulated.as_secs();

        if let Some(target) = current_target
            && elapsed_secs >= target {
                if !app.mute {
                    print!("\x07"); // Beep
                }

                let should_exit = app.on_timer_complete();
                if should_exit {
                    break;
                } else {
                    // Reset for next phase
                    time_accumulated = Duration::ZERO;
                    last_tick = Instant::now();
                    current_target = app.get_target_duration();
                }
            }

        draw_simple_ui(app, elapsed_secs, current_target)?;
    }
    Ok(())
}

fn draw_simple_ui(app: &App, elapsed: u64, target: Option<u64>) -> io::Result<()> {
    // Calculate display time (Count down or Count up)
    let display_value = match target {
        Some(t) => t.saturating_sub(elapsed),
        None => elapsed,
    };

    // Status text logic
    let status_text = match &app.mode {
        AppMode::Chrono => String::new(),
        AppMode::Timer { .. } => String::new(),
        AppMode::Clock { .. } => String::new(),
        AppMode::Pomodoro { phase, cycles, .. } => {
            let phase_name = match phase {
                PomodoroPhase::Focus => "Focus",
                PomodoroPhase::ShortBreak => "Short Break",
                PomodoroPhase::LongBreak => "Long Break",
            };
            format!("#{} - {}", cycles + 1, phase_name)
        }
    };

    let time_str = if let AppMode::Clock { show_seconds } = app.mode {
        let now = chrono::Local::now();
        let format = if show_seconds { "%H:%M:%S" } else { "%H:%M" };
        now.format(format).to_string()
    } else if let AppMode::Pomodoro { .. } = app.mode {
        let h = display_value / 3600;
        let m = (display_value % 3600) / 60;
        format!("{:02}:{:02}", h, m)
    } else {
        utils::format_time(display_value)
    };

    let paused_indicator = if app.paused { " [PAUSED]" } else { "" };
    let separator = if status_text.is_empty() { "" } else { " " };

    // \x1B[K clears the line residue
    print!(
        "\r{}{}[{}] {}{} \x1B[K",
        status_text,
        separator,
        time_str,
        paused_indicator,
        if app.paused { "" } else { " " }
    );

    io::stdout().flush()?;
    Ok(())
}
