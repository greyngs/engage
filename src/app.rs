#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PomodoroPhase {
    Focus,
    ShortBreak,
    LongBreak,
}

#[derive(Debug)]
pub enum AppMode {
    Chrono,
    Clock {
        show_seconds: bool,
    },
    Timer {
        target: u64,
    },
    Pomodoro {
        phase: PomodoroPhase,
        cycles: u32,
        config: PomodoroConfig,
    },
}

#[derive(Debug)]
pub struct PomodoroConfig {
    pub focus: u64,
    pub short: u64,
    pub long: u64,
}

pub struct App {
    pub mode: AppMode,
    pub paused: bool,
    pub mute: bool,
}

impl App {
    pub fn new(mode: AppMode, mute: bool) -> Self {
        Self {
            mode,
            paused: false,
            mute,
        }
    }

    /// Returns the target duration for the current state (if any).
    pub fn get_target_duration(&self) -> Option<u64> {
        match &self.mode {
            AppMode::Chrono | AppMode::Clock { .. } => None,
            AppMode::Timer { target } => Some(*target),
            AppMode::Pomodoro { phase, config, .. } => Some(match phase {
                PomodoroPhase::Focus => config.focus,
                PomodoroPhase::ShortBreak => config.short,
                PomodoroPhase::LongBreak => config.long,
            }),
        }
    }

    /// Handles the logic when the timer reaches zero (Pomodoro transitions).
    pub fn on_timer_complete(&mut self) -> bool {
        match &mut self.mode {
            AppMode::Timer { .. } => true, // Timer finishes the app
            AppMode::Chrono | AppMode::Clock { .. } => false, // Should not happen, but safe default
            AppMode::Pomodoro { phase, cycles, .. } => {
                match phase {
                    PomodoroPhase::Focus => {
                        // Focus finished. Decide break type.
                        let current_cycle = *cycles + 1;
                        if current_cycle % 4 == 0 {
                            *phase = PomodoroPhase::LongBreak;
                        } else {
                            *phase = PomodoroPhase::ShortBreak;
                        }
                    }
                    PomodoroPhase::ShortBreak | PomodoroPhase::LongBreak => {
                        *cycles += 1;
                        *phase = PomodoroPhase::Focus;
                    }
                }
                false
            }
        }
    }
}
