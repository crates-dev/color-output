use super::r#type::from_env_var;
use std::fmt::Write;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Leap Year
static LEAP_YEAR: [u64; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
/// Common Year
static COMMON_YEAR: [u64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

/// Determines if a year is a leap year.
///
/// # Parameters
/// `u64`: The year
///
/// # Returns
/// `bool`: Whether it is a leap year
fn is_leap_year(year: u64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// Gets the current time.
///
/// # Returns
/// `String`: The formatted time
pub fn get_now_time_format() -> String {
    // Get the current time
    let start: SystemTime = SystemTime::now();
    let duration: Duration = start.duration_since(UNIX_EPOCH).unwrap();
    let total_seconds: u64 = duration.as_secs();
    let mut total_days: u64 = total_seconds / 86400;
    // Calculate the years, considering leap years in between
    let mut year: u64 = 1970;
    while total_days >= if is_leap_year(year) { 366 } else { 365 } {
        total_days -= if is_leap_year(year) { 366 } else { 365 };
        year += 1;
    }
    // Calculate the month and day
    let mut month: i32 = 1;
    let month_days: [u64; 12] = if is_leap_year(year) {
        LEAP_YEAR
    } else {
        COMMON_YEAR
    };
    // Calculate the current month
    while total_days >= month_days[month as usize - 1] {
        total_days -= month_days[month as usize - 1];
        month += 1;
    }
    // Current date
    let day: u64 = total_days + 1;
    // Calculate remaining hours
    let remaining_seconds: u64 = total_seconds % 86400;
    let timezone_offset: u64 = from_env_var().value();
    let hours: u64 = ((remaining_seconds + timezone_offset) / 3600) % 24;
    let minutes: u64 = (remaining_seconds % 3600) / 60;
    let seconds: u64 = remaining_seconds % 60;
    // Create date string
    let mut date_time: String = String::new();
    write!(
        &mut date_time,
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        year, month, day, hours, minutes, seconds
    )
    .unwrap_or_default();
    date_time
}
