use crate::todo::{TodoError, TodoResult};
use chrono::{Local, NaiveDate, NaiveDateTime};

const DATE_FORMATS: &[&str] = &[
    "%d-%m-%Y %H:%M", // 25-12-2023 14:30
    "%d/%m/%Y %H:%M", // 25/12/2023 14:30
    "%Y-%m-%d %H:%M", // 2023-12-25 14:30
    "%d-%m-%Y",       // 25-12-2023 (will add 00:00)
    "%d/%m/%Y",       // 25/12/2023 (will add 00:00)
    "%Y-%m-%d",       // 2023-12-25 (will add 00:00)
];

pub fn parse_due_date(due_str: Option<&str>) -> TodoResult<Option<NaiveDateTime>> {
    let Some(date_str) = due_str else {
        return Ok(None);
    };

    let date_str = date_str.trim();
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
        return Ok(Some(dt));
    }

    Err(TodoError::InvalidDateFormat)
}

fn parse_relative_date(input: &str) -> TodoResult<Option<NaiveDateTime>> {
    let now = Local::now().naive_local();

    match input.to_lowercase().as_str() {
        "today" => Ok(Some(now.date().and_hms_opt(23, 59, 0).unwrap())),
        "tomorrow" => Ok(Some(
            (now.date() + chrono::Duration::days(1))
                .and_hms_opt(23, 59, 0)
                .unwrap(),
        )),
        input if input.ends_with("d") || input.ends_with(" days") => {
            let days_str = input.trim_end_matches("d").trim_end_matches(" days").trim();
            if let Ok(days) = days_str.parse::<i64>() {
                Ok(Some(
                    (now.date() + chrono::Duration::days(days))
                        .and_hms_opt(23, 59, 0)
                        .unwrap(),
                ))
            } else {
                Ok(None)
            }
        }
        _ => Ok(None),
    }
}
