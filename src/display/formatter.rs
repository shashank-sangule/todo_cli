use chrono::{Local, NaiveDateTime};

pub fn format_status(status: bool) -> &'static str {
    if status {
        "✅"
    } else {
        "⬜"
    }
}

pub fn format_due_date(due_date: Option<NaiveDateTime>) -> String {
    format_due_date_with_now(due_date, None)
}

pub fn format_due_date_with_now(
    due_date: Option<NaiveDateTime>,
    now: Option<NaiveDateTime>,
) -> String {
    match due_date {
        Some(due_date) => {
            let now = now.unwrap_or_else(|| Local::now().naive_local());
            let due_date_date = due_date.date();
            let diff = due_date_date.signed_duration_since(now.date());

            if diff.num_days() < 0 {
                format!("🔴 {} (overdue)", due_date.format("%d-%m-%Y %H:%M"))
            } else if diff.num_days() == 0 {
                format!("🟡 {} (today)", due_date.format("%H:%M"))
            } else if diff.num_days() == 1 {
                format!("🟢 {} (tomorrow)", due_date.format("%H:%M"))
            } else if diff.num_days() <= 7 {
                format!(
                    "🟢 {} ({} days)",
                    due_date.format("%d-%m %H:%M"),
                    diff.num_days()
                )
            } else {
                format!("⚪ {}", due_date.format("%d-%m-%Y"))
            }
        }
        None => "-".to_string(),
    }
}

pub fn truncate_text(text: &str, max_len: usize) -> String {
    if text.len() <= max_len {
        text.to_string()
    } else {
        format!("{}...", &text[..max_len.saturating_sub(3)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;

    // Helper to create test dates
    fn test_date(year: i32, month: u32, day: u32, hour: u32, min: u32) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(
            &format!("{year}-{month:02}-{day:02} {hour:02}:{min:02}:00"),
            "%Y-%m-%d %H:%M:%S",
        )
        .unwrap()
    }

    #[test]
    fn test_format_status_completed() {
        assert_eq!(format_status(true), "✅");
    }

    #[test]
    fn test_format_status_incomplete() {
        assert_eq!(format_status(false), "⬜");
    }

    #[test]
    fn test_format_due_date_none() {
        assert_eq!(format_due_date(None), "-");
    }

    #[test]
    fn test_format_due_date_overdue() {
        let past_date = test_date(2025, 8, 6, 0, 0); // Yesterday from current date (Aug 7, 2025)
        let result = format_due_date(Some(past_date));

        assert!(result.starts_with("🔴"));
        assert!(result.contains("06-08-2025 00:00"));
        assert!(result.contains("(overdue)"));
    }

    #[test]
    fn test_format_due_date_today() {
        let mock_now = test_date(2025, 8, 7, 8, 0); // Mock current time
        let today_date = test_date(2025, 8, 7, 10, 0); // Same day, different time
        let result = format_due_date_with_now(Some(today_date), Some(mock_now));

        assert!(result.starts_with("🟡"));
        assert!(result.contains("10:00"));
        assert!(result.contains("(today)"));
    }

    #[test]
    fn test_format_due_date_tomorrow() {
        let mock_now = test_date(2025, 8, 7, 8, 0); // Mock current time
        let tomorrow_date = test_date(2025, 8, 8, 15, 0); // Next day
        let result = format_due_date_with_now(Some(tomorrow_date), Some(mock_now));

        assert!(result.starts_with("🟢"));
        assert!(result.contains("15:00"));
        assert!(result.contains("(tomorrow)"));
    }

    #[test]
    fn test_format_due_date_within_week() {
        let week_date = test_date(2025, 8, 12, 12, 0); // 5 days from now
        let result = format_due_date(Some(week_date));

        assert!(result.starts_with("🟢"));
        assert!(result.contains("12-08 12:00"));
        assert!(result.contains("days)"));
    }

    #[test]
    fn test_format_due_date_beyond_week() {
        let future_date = test_date(2025, 8, 17, 10, 0); // 10 days from now
        let result = format_due_date(Some(future_date));

        assert!(result.starts_with("⚪"));
        assert!(result.contains("17-08-2025"));
        assert!(!result.contains("days"));
    }

    #[test]
    fn test_truncate_text_no_truncation_needed() {
        let text = "Short text";
        assert_eq!(truncate_text(text, 20), "Short text");
    }

    #[test]
    fn test_truncate_text_exact_length() {
        let text = "Exactly ten!!"; // 13 chars
        assert_eq!(truncate_text(text, 13), "Exactly ten!!");
    }

    #[test]
    fn test_truncate_text_truncation_needed() {
        let text = "This is a very long text that needs truncation";
        let result = truncate_text(text, 20);

        assert_eq!(result, "This is a very lo...");
        assert_eq!(result.len(), 20);
    }

    #[test]
    fn test_truncate_text_short_max_length() {
        let text = "Hello world";
        let result = truncate_text(text, 5);

        assert_eq!(result, "He...");
        assert_eq!(result.len(), 5);
    }
}
