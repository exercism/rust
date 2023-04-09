pub enum LogLevel {
    Unknown = 0,
    Trace = 1,
    Debug = 2,
    Info = 4,
    Warning = 5,
    Error = 6,
    Fatal = 42,
}

pub fn parse_log_level(log_line: &str) -> LogLevel {
    if log_line.starts_with("[TRC]") {
        return LogLevel::Trace;
    }
    if log_line.starts_with("[DBG]") {
        return LogLevel::Debug;
    }
    if log_line.starts_with("[INF]") {
        return LogLevel::Info;
    }
    if log_line.starts_with("[WRN]") {
        return LogLevel::Warning;
    }
    if log_line.starts_with("[ERR]") {
        return LogLevel::Error;
    }
    if log_line.starts_with("[FTL]") {
        return LogLevel::Fatal;
    }
    return LogLevel::Unknown;
}

pub fn output_for_short_log(log_level: LogLevel, message: &str) -> String {
    return format!("{}:{}", log_level as i32, message);
}
