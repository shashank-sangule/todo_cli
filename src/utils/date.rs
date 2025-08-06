use crate::todo::{TodoError, TodoResult};
use chrono::{Local, NaiveDate, NaiveDateTime};
use chrono_english::{parse_date_string, Dialect};

const DATE_FORMATS: &[&str] = &[
    "%d-%m-%Y %H:%M", // 25-12-2023 14:30
    "%d/%m/%Y %H:%M", // 25/12/2023 14:30
    "%Y-%m-%d %H:%M", // 2023-12-25 14:30
    "%d-%m-%Y",       // 25-12-2023 (will add 00:00)
    "%d/%m/%Y",       // 25/12/2023 (will add 00:00)
    "%Y-%m-%d",       // 2023-12-25 (will add 00:00)
    "%Y/%m/%d",       // 2023/12/25 (will add 00:00)
];

pub fn parse_due_date(due_str: Option<&str>) -> TodoResult<Option<NaiveDateTime>> {
    let Some(date_str) = due_str else {
        return Ok(None);
    };

    let date_str = date_str.trim().to_lowercase();
    let date_str = date_str.as_str();
    if date_str.is_empty() {
        return Ok(None);
    }

    for format in DATE_FORMATS.iter().filter(|f| f.contains("%H:%M")) {
        if let Ok(dt) = NaiveDateTime::parse_from_str(date_str, format) {
            return Ok(Some(dt));
        }
    }

    for format in DATE_FORMATS.iter().filter(|f| !f.contains("%H:%M")) {
        if let Ok(date) = NaiveDate::parse_from_str(date_str, format) {
            return Ok(Some(date.and_hms_opt(0, 0, 0).unwrap()));
        }
    }

    if let Some(dt) = parse_relative_date(date_str)? {
        if dt < Local::now().naive_local() {
            return Err(TodoError::InvalidDate {
                input: date_str.to_string(),
                reason: "Date cannot be in the past".to_string(),
            });
        }
        return Ok(Some(dt));
    }

    Err(TodoError::InvalidDateFormat {
        input: date_str.to_string(),
    })
}

fn parse_relative_date(input: &str) -> TodoResult<Option<NaiveDateTime>> {
    let now = Local::now().naive_local();

    match input {
        "tomorrow" => {
            let tomorrow = now.date() + chrono::Duration::days(1);
            Ok(Some(tomorrow.and_hms_opt(0, 0, 0).unwrap()))
        }
        "yesterday" => {
            let yesterday = now.date() - chrono::Duration::days(1);
            Ok(Some(yesterday.and_hms_opt(0, 0, 0).unwrap()))
        }
        "next week" => {
            let next_week = now + chrono::Duration::weeks(1);
            Ok(Some(next_week.date().and_hms_opt(0, 0, 0).unwrap()))
        }
        "last week" => {
            let last_week = now - chrono::Duration::weeks(1);
            Ok(Some(last_week.date().and_hms_opt(0, 0, 0).unwrap()))
        }
        _ => {
            let dialect = Dialect::Uk;
            match parse_date_string(input, Local::now(), dialect) {
                Ok(date) => {
                    let naive_date = date.naive_utc().date();
                    let naive_datetime =
                        NaiveDateTime::new(naive_date, NaiveDateTime::default().time());
                    Ok(Some(naive_datetime))
                }
                Err(_) => Ok(None),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, Timelike};

    #[test]
    fn test_none_and_empty_input() {
        assert_eq!(parse_due_date(None).unwrap(), None);
        assert_eq!(parse_due_date(Some("")).unwrap(), None);
        assert_eq!(parse_due_date(Some("   ")).unwrap(), None);
    }

    #[test]
    fn test_absolute_dates_with_time() {
        let result = parse_due_date(Some("25-12-2025 14:30")).unwrap();
        assert!(result.is_some());
        let dt = result.unwrap();
        assert_eq!(dt.date(), NaiveDate::from_ymd_opt(2025, 12, 25).unwrap());
        assert_eq!(dt.time().hour(), 14);
        assert_eq!(dt.time().minute(), 30);

        let result = parse_due_date(Some("2025-12-25 09:15")).unwrap();
        assert!(result.is_some());
        let dt = result.unwrap();
        assert_eq!(dt.date(), NaiveDate::from_ymd_opt(2025, 12, 25).unwrap());
        assert_eq!(dt.time().hour(), 9);
        assert_eq!(dt.time().minute(), 15);
    }

    #[test]
    fn test_absolute_dates_without_time() {
        let result = parse_due_date(Some("25-12-2025")).unwrap();
        assert!(result.is_some());
        let dt = result.unwrap();
        assert_eq!(dt.date(), NaiveDate::from_ymd_opt(2025, 12, 25).unwrap());
        assert_eq!(dt.time().hour(), 0);
        assert_eq!(dt.time().minute(), 0);

        let result = parse_due_date(Some("2025/12/25")).unwrap();
        assert!(result.is_some());
        let dt = result.unwrap();
        assert_eq!(dt.date(), NaiveDate::from_ymd_opt(2025, 12, 25).unwrap());
        assert_eq!(dt.time().hour(), 0);
        assert_eq!(dt.time().minute(), 0);
    }

    #[test]
    fn test_relative_dates_future() {
        let result = parse_due_date(Some("tomorrow"));
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());

        let result = parse_due_date(Some("next week"));
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());

        let result = parse_due_date(Some("3 days"));
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[test]
    fn test_relative_dates_past_rejected() {
        let result = parse_due_date(Some("yesterday"));
        assert!(result.is_err());
        match result.unwrap_err() {
            TodoError::InvalidDate { input, reason } => {
                assert_eq!(input, "yesterday");
                assert!(reason.contains("past"));
            }
            _ => panic!("Expected InvalidDate error"),
        }

        let result = parse_due_date(Some("last week"));
        assert!(result.is_err());
        match result.unwrap_err() {
            TodoError::InvalidDate { input, reason } => {
                assert_eq!(input, "last week");
                assert!(reason.contains("past"));
            }
            _ => panic!("Expected InvalidDate error"),
        }
    }

    #[test]
    fn test_invalid_formats() {
        let result = parse_due_date(Some("invalid date"));
        assert!(result.is_err());
        match result.unwrap_err() {
            TodoError::InvalidDateFormat { input } => {
                assert_eq!(input, "invalid date");
            }
            _ => panic!("Expected InvalidDateFormat error"),
        }

        let result = parse_due_date(Some("32-13-2025"));
        assert!(result.is_err());
        match result.unwrap_err() {
            TodoError::InvalidDateFormat { input } => {
                assert_eq!(input, "32-13-2025");
            }
            _ => panic!("Expected InvalidDateFormat error"),
        }

        let result = parse_due_date(Some("2025-02-30"));
        assert!(result.is_err());
        match result.unwrap_err() {
            TodoError::InvalidDateFormat { input } => {
                assert_eq!(input, "2025-02-30");
            }
            _ => panic!("Expected InvalidDateFormat error"),
        }
    }

    #[test]
    fn test_edge_cases() {
        let result = parse_due_date(Some("  25-12-2025 14:30  ")).unwrap();
        assert!(result.is_some());
        let dt = result.unwrap();
        assert_eq!(dt.date(), NaiveDate::from_ymd_opt(2025, 12, 25).unwrap());

        let result = parse_due_date(Some("TOMORROW"));
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[test]
    fn test_parse_relative_date_internal() {
        let result = parse_relative_date("not a date at all xyz123");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);

        let result = parse_relative_date("tomorrow");
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }
}
