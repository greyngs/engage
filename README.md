# Engage

**Make it so.**

A simple, straightforward terminal tool to track my time. No distractions, just a clean interface for Pomodoro, timers, and stopwatches.

![Pomodoro Demo](pomodoro.gif)

## Features

- **Timer**: Flexible countdown (e.g., `1h 30m`, `15m`, `5s`).
- **Pomodoro**: Focus/Break cycle tracker (Focus -> Short Break -> Focus -> Long Break).
- **Stopwatch**: Simple elapsed time tracker.
- **Clock**: Large digital clock display with optional seconds.
- **Two Visual Modes**:
    - **TUI**: Rich interface with large text (Block characters or ASCII art).
    - **Simple**: Minimalist line-based output for low-distraction or scripts.

## Installation

### From crates.io (Recommended)

```bash
cargo install engage-cli
```
*Note: The binary name is `engage`, so you can run it simply by typing `engage`.*

### Build from Source

```bash
git clone https://github.com/greyngs/engage
cd engage
cargo install --path .
```

## Usage

```bash
engage <COMMAND> [OPTIONS]
```

### Commands

| Command | Description | Example |
| :--- | :--- | :--- |
| `timer` | Start a countdown timer. Accepts duration strings like `10m`, `1h 30s`. | `engage timer 25m` |
| `pomodoro` | Start a Pomodoro session. Defaults to 25m focus, 5m short break, 15m long break. | `engage pomodoro` |
| `chrono` | Start a stopwatch (count-up timer). | `engage chrono` |
| `clock` | Display the current local time. | `engage clock` |

### Options

| Flag | commands | Description |
| :--- | :--- | :--- |
| `--ascii` | *All* | Use ASCII art font instead of block characters (Retro style). |
| `--simple` | *All* | Run in simple text mode (standard output, no TUI). |
| `--mute` | *All* | Disable sound alerts on completion. |
| `--focus <N>` | `pomodoro` | Set custom focus duration in minutes (default: 25). |
| `--short <N>` | `pomodoro` | Set custom short break duration in minutes (default: 5). |
| `--long <N>` | `pomodoro` | Set custom long break duration in minutes (default: 15). |
| `--seconds` | `clock` | Show seconds in the clock display. |
| `-h`, `--help` | *All* | Show help message. |

### Controls (TUI Mode)

| Key | Action |
| :---: | :--- |
| `q` | Quit application |
| `Space` | Pause / Resume |
| `r` | Reset timer/stopwatch |

## Examples

**Standard Timer**
```bash
engage timer 10m 30s
```

**Custom Pomodoro**
```bash
engage pomodoro --focus 50 --short 10
```

**Clock**
```bash
engage clock --ascii --seconds
```

**Silent Simple Timer**
```bash
engage timer 5m --simple --mute
```

## License

MIT
