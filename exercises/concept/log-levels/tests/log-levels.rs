#[test]
pub fn error_message() {
    assert_eq!(
        "Stack overflow",
        log_levels::message("[ERROR]: Stack overflow")
    );
}

#[test]
#[ignore]
pub fn warning_message() {
    assert_eq!(
        "Disk almost full",
        log_levels::message("[WARNING]: Disk almost full"),
    );
}

#[test]
#[ignore]
pub fn info_message() {
    assert_eq!("File moved", log_levels::message("[INFO]: File moved"));
}

#[test]
#[ignore]
pub fn message_with_leading_and_trailing_white_space() {
    assert_eq!(
        "Timezone not set",
        log_levels::message("[WARNING]:   \tTimezone not set  \r\n"),
    );
}

#[test]
#[ignore]
pub fn error_log_level() {
    assert_eq!("error", log_levels::log_level("[ERROR]: Disk full"));
}

#[test]
#[ignore]
pub fn warning_log_level() {
    assert_eq!(
        "warning",
        log_levels::log_level("[WARNING]: Unsafe password")
    );
}

#[test]
#[ignore]
pub fn info_log_level() {
    assert_eq!("info", log_levels::log_level("[INFO]: Timezone changed"));
}

#[test]
#[ignore]
pub fn error_reformat() {
    assert_eq!(
        "Segmentation fault (error)",
        log_levels::reformat("[ERROR]: Segmentation fault"),
    );
}

#[test]
#[ignore]
pub fn warning_reformat() {
    assert_eq!(
        "Decreased performance (warning)",
        log_levels::reformat("[WARNING]: Decreased performance"),
    );
}

#[test]
#[ignore]
pub fn info_reformat() {
    assert_eq!(
        "Disk defragmented (info)",
        log_levels::reformat("[INFO]: Disk defragmented"),
    );
}

#[test]
#[ignore]
pub fn reformat_with_leading_and_trailing_white_space() {
    assert_eq!(
        "Corrupt disk (error)",
        log_levels::reformat("[ERROR]: \t Corrupt disk\t \t \r\n"),
    );
}
