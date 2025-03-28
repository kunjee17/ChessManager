use chrono::DateTime;
/**
 * Format the date in the format of "Mar 23, 2025"
 * Chess in name to avoid conflict with the format_date function
 */
pub fn format_date_chess(date: &str) -> String {
    let date = DateTime::parse_from_rfc3339(date).unwrap();
    date.format("%b %d, %Y").to_string()
}
/**
 * Format the date in the format of "Mar 23, 2025 12:00:00"
 * Chess in name to avoid conflict with the format_date_time function
 */
pub fn format_date_time_chess(date: &str) -> String {
    let date = DateTime::parse_from_rfc3339(date).unwrap();
    date.format("%b %d, %Y %H:%M:%S").to_string()
}
