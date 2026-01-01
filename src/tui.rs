use std::io;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Alignment},
    widgets::{Block, Borders, Paragraph},
    style::{Style, Color},
    Terminal,
};
use tui_big_text::{BigText, PixelSize};
use figlet_rs::FIGfont;
use std::time::{Duration, Instant};

use crate::app::{App, AppMode, PomodoroPhase};
use crate::utils;

pub fn run_tui(app: &mut App, use_ascii: bool) -> io::Result<()> {
    // Setup terminal
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let res = run_loop(&mut terminal, app, use_ascii);

    terminal.show_cursor()?;
    terminal.clear()?;

    res
}

fn run_loop(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
    use_ascii: bool,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    let mut current_target = app.get_target_duration();
    let mut time_accumulated = Duration::ZERO;

    loop {
        terminal.draw(|f| {
            let size = f.area();
            
            // Layout: Timer takes full space mainly, Status at bottom
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(5),
                    Constraint::Length(1),
                ])
                .split(size);

            let status_text = if let AppMode::Pomodoro { phase, cycles, .. } = &app.mode {
                let phase_name = match phase {
                    PomodoroPhase::Focus => "FOCUS",
                    PomodoroPhase::ShortBreak => "SHORT BREAK",
                    PomodoroPhase::LongBreak => "LONG BREAK",
                };
                format!("POMODORO #{} - {}", cycles + 1, phase_name)
            } else {
                String::new()
            };
            
            let paused_text = if app.paused { " [PAUSED]" } else { "" };
            let full_status = if status_text.is_empty() {
                paused_text.to_string()
            } else {
                format!("{}{}", status_text, paused_text)
            };
            
            let elapsed = time_accumulated.as_secs();
            let display_value = match current_target {
                Some(t) => t.saturating_sub(elapsed),
                None => elapsed,
            };

            let time_str = if let AppMode::Clock { show_seconds } = app.mode {
                let now = chrono::Local::now();
                let format = if show_seconds { "%H:%M:%S" } else { "%H:%M" };
                now.format(format).to_string()
            } else if let AppMode::Pomodoro { .. } = app.mode {
                let h = display_value / 3600;
                let m = (display_value % 3600) / 60;
                let _s = display_value % 60; // Seconds ignored
                format!("{:02}:{:02}", h, m)
            } else {
                utils::format_time(display_value)
            };

            if use_ascii {
                // ASCII Mode using Roman Figlet font
                let font_text = include_str!("fonts/Basic.flf");
                let standard_font = FIGfont::from_content(font_text).unwrap();
                let figure = standard_font.convert(&time_str);
                let ascii_art = figure.map(|f| f.to_string()).unwrap_or(time_str.clone());
                
                let timer_display = Paragraph::new(ascii_art)
                    .alignment(Alignment::Center)
                    .block(Block::default().borders(Borders::NONE));
                
                let v_center = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Percentage(30),
                        Constraint::Min(5), 
                        Constraint::Percentage(30),
                    ])
                    .split(chunks[0]);
                    
                f.render_widget(timer_display, v_center[1]);

            } else {
                let big_text = BigText::builder()
                    .pixel_size(PixelSize::Full)
                    .style(Style::default().fg(Color::Gray))
                    .lines(vec![time_str.clone().into()])
                    .alignment(Alignment::Center)
                    .build();
                
                let v_center = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Percentage(30),
                        Constraint::Min(5), 
                        Constraint::Percentage(30),
                    ])
                    .split(chunks[0]);
                
                 f.render_widget(big_text, v_center[1]);
            }

            let footer = Paragraph::new(full_status)
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::DarkGray));
            f.render_widget(footer, chunks[1]);

        })?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
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
        }

        // Time Logic
        if !app.paused {
            let now = Instant::now();
            if now.duration_since(last_tick) >= Duration::from_secs(1) {
                 time_accumulated += now.duration_since(last_tick);
                 last_tick = now;
            } else {
                 time_accumulated += now.duration_since(last_tick);
                 last_tick = now;
            }
        }

        let elapsed_secs = time_accumulated.as_secs();

        if let Some(target) = current_target {
            if elapsed_secs >= target {
                if !app.mute {
                    // Beep
                    print!("\x07"); 
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
        }
    }
    Ok(())
}
