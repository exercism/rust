use logs_logs_logs::{output_for_short_log, parse_log_level, LogLevel};

#[test]
pub fn parse_trace() {
    assert_eq!(
        LogLevel::Trace as i32,
        parse_log_level("[TRC]: Line 84 - Console.WriteLine('Hello World');") as i32
    );
}

#[test]
#[ignore]
pub fn parse_debug() {
    assert_eq!(
        LogLevel::Debug as i32,
        parse_log_level("[DBG]: ; expected") as i32
    );
}

#[test]
#[ignore]
pub fn parse_info() {
    assert_eq!(
        LogLevel::Info as i32,
        parse_log_level("[INF]: Timezone changed") as i32
    );
}

#[test]
#[ignore]
pub fn parse_warning() {
    assert_eq!(
        LogLevel::Warning as i32,
        parse_log_level("[WRN]: Timezone not set") as i32
    );
}

#[test]
#[ignore]
pub fn parse_error() {
    assert_eq!(
        LogLevel::Error as i32,
        parse_log_level("[ERR]: Disk full") as i32
    );
}

#[test]
#[ignore]
pub fn parse_fatal() {
    assert_eq!(
        LogLevel::Fatal as i32,
        parse_log_level("[FTL]: Not enough memory") as i32
    );
}

#[test]
#[ignore]
pub fn parse_unknown() {
    assert_eq!(
        LogLevel::Unknown as i32,
        parse_log_level("[XYZ]: Gibberish message.. beep boop..") as i32
    );
}

#[test]
#[ignore]
pub fn output_for_short_log_for_trace() {
    assert_eq!(
        "1:Line 13 - int myNum = 42;",
        output_for_short_log(LogLevel::Trace, "Line 13 - int myNum = 42;")
    );
}

#[test]
#[ignore]
pub fn output_for_short_log_for_debug() {
    assert_eq!(
        "2:The name 'LogLevel' does not exist in the current context",
        output_for_short_log(
            LogLevel::Debug,
            "The name 'LogLevel' does not exist in the current context"
        )
    );
}

#[test]
#[ignore]
pub fn output_for_short_log_for_info() {
    assert_eq!(
        "4:File moved",
        output_for_short_log(LogLevel::Info, "File moved")
    );
}

#[test]
#[ignore]
pub fn output_for_short_log_for_warning() {
    assert_eq!(
        "5:Unsafe password",
        output_for_short_log(LogLevel::Warning, "Unsafe password")
    );
}

#[test]
#[ignore]
pub fn output_for_short_log_for_error() {
    assert_eq!(
        "6:Stack overflow",
        output_for_short_log(LogLevel::Error, "Stack overflow")
    );
}

#[test]
#[ignore]
pub fn output_for_short_log_for_fatal() {
    assert_eq!(
        "42:Dumping all files",
        output_for_short_log(LogLevel::Fatal, "Dumping all files")
    );
}

#[test]
#[ignore]
pub fn output_for_short_log_for_unknown() {
    assert_eq!(
        "0:Something unknown happened",
        output_for_short_log(LogLevel::Unknown, "Something unknown happened")
    );
}
