use chrono::{Local, NaiveDateTime};

pub fn format_status(status: bool) -> &'static str {
    if status { "✅" } else { "⬜" }
}

pub fn format_due_date(due: Option<NaiveDateTime>) -> String {
    match due {
        Some(due) => {
            let now = Local::now().naive_local();
            let diff = due.signed_duration_since(now);

            if diff.num_days() < 0 {
                format!("🔴 {} (overdue)", due.format("%d-%m-%Y %H:%M"))
            } else if diff.num_days() == 0 {
                format!("🟡 {} (today)", due.format("%H:%M"))
            } else if diff.num_days() == 1 {
                format!("🟢 {} (tomorrow)", due.format("%H:%M"))
            } else if diff.num_days() <= 7 {
                format!(
                    "🟢 {} ({} days)",
                    due.format("%d-%m %H:%M"),
                    diff.num_days()
                )
            } else {
                format!("⚪ {}", due.format("%d-%m-%Y"))
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
