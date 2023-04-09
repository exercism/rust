pub enum LogLevel {
    Unknown,
    Trace,
    Debug,
    Info,
    Warning,
    Error,
    Fatal,
}

pub fn parse_log_level(_log_line: &str) -> LogLevel {
    unimplemented!("Implement parse_log_level")
}

pub fn output_for_short_log(_log_level: LogLevel, _message: &str) -> String {
    unimplemented!("Implement output_for_short_log")
}
