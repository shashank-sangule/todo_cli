pub fn parse_due_date(due_str: Option<&str>) -> TodoResult<Option<NaiveDateTime>> {
    match due_str {
        Some(date_str) if !date_str.trim().is_empty() => {
            let date_str = date_str.trim();

            for format in DATE_FORMATS {
                if let Ok(dt) = NaiveDateTime::parse_from_str(date_str, format) {
                    return Ok(Some(dt));
                }

                // Try parsing as date-only and add default time (00:00)
                if format.contains(" %H:%M") {
                    let date_format = format.replace(" %H:%M", "");
                    if let Ok(date) = NaiveDate::parse_from_str(date_str, &date_format) {
                        if let Some(datetime) = date.and_hms_opt(0, 0, 0) {
                            return Ok(Some(datetime));
                        }
                    }
                }
            }
            Err(TodoError::InvalidDateFormat)
        }
        _ => Ok(None),
    }
}
