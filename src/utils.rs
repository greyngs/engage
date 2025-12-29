pub fn parse_duration_from_args(args: &[String]) -> u64 {
    let mut total_seconds = 0;
    for arg in args {
        let mut t = arg.clone();
        let unit = t.pop().unwrap_or('s'); // Default to seconds if no unit
        let value: u64 = t.parse().unwrap_or(0);

        total_seconds += match unit {
            'h' => value * 3600,
            'm' => value * 60,
            _ => value,
        };
    }
    total_seconds
}

/// Formats seconds into HH:MM:SS string.
pub fn format_time(seconds: u64) -> String {
    let h = seconds / 3600;
    let m = (seconds % 3600) / 60;
    let s = seconds % 60;
    format!("{:02}:{:02}:{:02}", h, m, s)
}
