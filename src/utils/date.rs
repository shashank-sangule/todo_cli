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
    let now = Local::now();

    let dialect = Dialect::Uk; // Use UK dialect for date parsing
    match parse_date_string(input, now, dialect) {
        Ok(date) => {
            let naive_date = date.naive_utc().date();
            let naive_datetime = NaiveDateTime::new(naive_date, NaiveDateTime::default().time());
            Ok(Some(naive_datetime))
        }
        Err(e) => {
            if e.to_string().contains("no date found") {
                return Ok(None);
            }
            Err(TodoError::InvalidDateFormat {
                input: input.to_string(),
            })
        }
    }
}
