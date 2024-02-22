use chrono::{DateTime, Datelike, NaiveDate, Utc};

/// Formats a [DateTime] instance to `HH:MM`
///
/// # Arguments
/// * - `time` - The time that is going to be formatted
///
/// # Returns:
/// The formatted time
pub(crate) fn format_time(time: &DateTime<Utc>) -> String {
    time.format("%H:%M").to_string()
}

/// Formats a [NaiveDate] instance to `dd/nm/YYYY"`
///
/// # Arguments
/// * - `date` - The date that is going to be formatted
///
/// # Returns:
/// The formatted date
pub(crate) fn format_date(date: Option<&NaiveDate>) -> String {
    match date {
        Some(date) => date.format("%d/%m/%Y").to_string(),
        None => "".to_string(),
    }
}

/// Formats a [DateTime] instance to an ISO 8601 format
///
/// # Arguments
/// * - `datetime` - The datetime that is going to be formatted
///
/// # Returns:
/// The formatted datetime
pub(crate) fn format_datetime(datetime: Option<&DateTime<Utc>>) -> String {
    match datetime {
        Some(datetime) => datetime.format("%Y-%m-%dT%H:%M:%S").to_string(),
        None => "".to_string(),
    }
}

/// Returns the day of the week from a [NaiveDate] instance
///
/// # Arguments
/// * - `date` - The date that is going to be evaluated
///
/// # Returns:
/// The day of the week
pub(crate) fn get_day_of_week(date: Option<NaiveDate>) -> String {
    match date {
        Some(date) => date.weekday().to_string(),
        None => "".to_string(),
    }
}

/// Returns the ergani value for overtime_cancellation
///
/// # Arguments
/// * - `overtime_cancellation` - A value representing if an overtime is going to to be cancelled or not
///
/// # Returns:
/// The Ergani value that is going to be used with the Ergani API
pub(crate) fn get_ergani_overtime_cancellation(cancellation: bool) -> String {
    if cancellation {
        "1".to_string()
    } else {
        "Ο".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, Utc};

    #[test]
    fn test_format_time() {
        let dt = Utc::now();
        let formatted_time = format_time(&dt);
        assert_eq!(formatted_time, dt.format("%H:%M").to_string());
    }

    #[test]
    fn test_format_date() {
        let date = NaiveDate::from_ymd_opt(2021, 1, 1).unwrap();
        let formatted_date = format_date(Some(&date));
        assert_eq!(formatted_date, date.format("%d/%m/%Y").to_string());
    }

    #[test]
    fn test_format_datetime() {
        let dt = Utc::now();
        let formatted_datetime = format_datetime(Some(&dt));
        assert_eq!(
            formatted_datetime,
            dt.format("%Y-%m-%dT%H:%M:%S").to_string()
        );
    }

    #[test]
    fn test_get_day_of_week() {
        let date = NaiveDate::from_ymd_opt(2021, 1, 1).unwrap();
        let day_of_week = get_day_of_week(Some(date));
        assert_eq!(day_of_week, date.weekday().to_string());
    }

    #[test]
    fn test_get_ergani_overtime_cancellation() {
        let cancellation = true;
        let ergani_cancellation = get_ergani_overtime_cancellation(cancellation);
        assert_eq!(ergani_cancellation, "1");
    }
}
