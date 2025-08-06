use crate::todo::error::TodoError;
use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TodoItem {
    id: u32,
    title: String,
    description: Option<String>,
    completed: bool,
    due_date: Option<NaiveDateTime>,
    priority: Option<Priority>,
    tags: Option<Vec<String>>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl TodoItem {
    pub fn new(
        id: u32,
        title: String,
        description: Option<String>,
        completed: bool,
        due_date: Option<NaiveDateTime>,
        priority: Option<Priority>,
        tags: Option<Vec<String>>,
    ) -> Self {
        let now = Local::now().naive_local();

        TodoItem {
            id,
            title,
            description,
            completed,
            due_date,
            priority,
            tags,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn completed(&self) -> bool {
        self.completed
    }

    pub fn due_date(&self) -> Option<NaiveDateTime> {
        self.due_date
    }

    pub fn priority(&self) -> Option<Priority> {
        self.priority
    }

    pub fn tags(&self) -> Option<&[String]> {
        self.tags.as_deref()
    }

    pub fn tags_string(&self) -> Option<String> {
        self.tags.as_ref().map(|t| t.join(", "))
    }

    pub fn is_overdue(&self) -> bool {
        if let Some(due) = self.due_date {
            due + chrono::Duration::minutes(5) < Local::now().naive_local() && !self.completed
        } else {
            false
        }
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
        self.touch();
    }

    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
        self.touch();
    }

    pub fn set_completed(&mut self, completed: bool) {
        self.completed = completed;
        self.touch();
    }

    pub fn set_due_date(&mut self, due_date: Option<NaiveDateTime>) {
        self.due_date = due_date;
        self.touch();
    }

    pub fn set_priority(&mut self, priority: Option<Priority>) {
        self.priority = priority;
        self.touch();
    }

    pub fn set_tags(&mut self, tags: Option<Vec<String>>) {
        self.tags = tags;
        self.touch();
    }

    fn touch(&mut self) {
        self.updated_at = Local::now().naive_local();
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl FromStr for Priority {
    type Err = TodoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "high" => Ok(Priority::High),
            "medium" => Ok(Priority::Medium),
            "low" => Ok(Priority::Low),
            _ => Err(TodoError::InvalidPriority {
                input: s.to_string(),
            }),
        }
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Priority::High => write!(f, "🔴 High"),
            Priority::Medium => write!(f, "🟡 Medium"),
            Priority::Low => write!(f, "🟢 Low"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum SortBy {
    Due,
    Priority,
    DueThenPriority,
}

impl FromStr for SortBy {
    type Err = TodoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "due" => Ok(SortBy::Due),
            "priority" => Ok(SortBy::Priority),
            "due+priority" => Ok(SortBy::DueThenPriority),
            _ => Err(TodoError::InvalidSortField {
                field: s.to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Local, NaiveDate};

    fn create_test_todo() -> TodoItem {
        TodoItem::new(
            1,
            "Test Todo".to_string(),
            Some("Test description".to_string()),
            false,
            Some(
                NaiveDate::from_ymd_opt(2024, 12, 25)
                    .unwrap()
                    .and_hms_opt(14, 30, 0)
                    .unwrap(),
            ),
            Some(Priority::High),
            Some(vec!["work".to_string(), "urgent".to_string()]),
        )
    }

    fn create_minimal_todo() -> TodoItem {
        TodoItem::new(1, "Minimal Todo".to_string(), None, false, None, None, None)
    }

    #[test]
    fn test_todo_item_creation() {
        let todo = create_test_todo();

        assert_eq!(todo.id(), 1);
        assert_eq!(todo.title(), "Test Todo");
        assert_eq!(todo.description(), Some("Test description"));
        assert!(!todo.completed());
        assert!(todo.due_date().is_some());
        assert_eq!(todo.priority(), Some(Priority::High));
        assert_eq!(
            todo.tags(),
            Some(["work".to_string(), "urgent".to_string()].as_slice())
        );
    }

    #[test]
    fn test_minimal_todo_creation() {
        let todo = create_minimal_todo();

        assert_eq!(todo.id(), 1);
        assert_eq!(todo.title(), "Minimal Todo");
        assert_eq!(todo.description(), None);
        assert!(!todo.completed());
        assert!(todo.due_date().is_none());
        assert_eq!(todo.priority(), None);
        assert_eq!(todo.tags(), None);
    }

    #[test]
    fn test_timestamps_are_set() {
        let before = Local::now().naive_local();
        let todo = create_test_todo();
        let after = Local::now().naive_local();

        assert!(todo.created_at >= before);
        assert!(todo.created_at <= after);
        assert!(todo.updated_at >= before);
        assert!(todo.updated_at <= after);
        assert_eq!(todo.created_at, todo.updated_at);
    }

    #[test]
    fn test_getters() {
        let todo = create_test_todo();

        assert_eq!(todo.id(), 1);
        assert_eq!(todo.title(), "Test Todo");
        assert_eq!(todo.description(), Some("Test description"));
        assert!(!todo.completed());
        assert!(todo.due_date().is_some());
        assert_eq!(todo.priority(), Some(Priority::High));
        assert_eq!(
            todo.tags(),
            Some(["work".to_string(), "urgent".to_string()].as_slice())
        );
    }

    #[test]
    fn test_is_overdue() {
        let past_date = Local::now().naive_local() - chrono::Duration::days(1);
        let overdue_todo = TodoItem::new(
            1,
            "Overdue Todo".to_string(),
            None,
            false,
            Some(past_date),
            None,
            None,
        );
        assert!(overdue_todo.is_overdue());

        let completed_overdue = TodoItem::new(
            2,
            "Completed Overdue".to_string(),
            None,
            true,
            Some(past_date),
            None,
            None,
        );
        assert!(!completed_overdue.is_overdue());

        let future_date = Local::now().naive_local() + chrono::Duration::days(1);
        let future_todo = TodoItem::new(
            3,
            "Future Todo".to_string(),
            None,
            false,
            Some(future_date),
            None,
            None,
        );
        assert!(!future_todo.is_overdue());

        let no_due_date =
            TodoItem::new(4, "No Due Date".to_string(), None, false, None, None, None);
        assert!(!no_due_date.is_overdue());
    }

    #[test]
    fn test_set_title() {
        let mut todo = create_test_todo();
        let original_updated_at = todo.updated_at;

        std::thread::sleep(std::time::Duration::from_millis(1));

        todo.set_title("New Title".to_string());

        assert_eq!(todo.title(), "New Title");
        assert!(todo.updated_at > original_updated_at);
    }

    #[test]
    fn test_set_description() {
        let mut todo = create_test_todo();
        let original_updated_at = todo.updated_at;

        std::thread::sleep(std::time::Duration::from_millis(1));

        todo.set_description(Some("New description".to_string()));
        assert_eq!(todo.description(), Some("New description"));
        assert!(todo.updated_at > original_updated_at);

        let updated_at_after_first_change = todo.updated_at;
        std::thread::sleep(std::time::Duration::from_millis(1));

        todo.set_description(None);
        assert_eq!(todo.description(), None);
        assert!(todo.updated_at > updated_at_after_first_change);
    }

    #[test]
    fn test_set_completed() {
        let mut todo = create_test_todo();
        let original_updated_at = todo.updated_at;

        std::thread::sleep(std::time::Duration::from_millis(1));

        todo.set_completed(true);
        assert!(todo.completed());
        assert!(todo.updated_at > original_updated_at);

        let updated_at_after_completion = todo.updated_at;
        std::thread::sleep(std::time::Duration::from_millis(1));

        todo.set_completed(false);
        assert!(!todo.completed());
        assert!(todo.updated_at > updated_at_after_completion);
    }

    #[test]
    fn test_set_due_date() {
        let mut todo = create_test_todo();
        let original_updated_at = todo.updated_at;
        let new_date = NaiveDate::from_ymd_opt(2025, 1, 1)
            .unwrap()
            .and_hms_opt(12, 0, 0)
            .unwrap();

        std::thread::sleep(std::time::Duration::from_millis(1));

        todo.set_due_date(Some(new_date));
        assert_eq!(todo.due_date(), Some(new_date));
        assert!(todo.updated_at > original_updated_at);

        let updated_at_after_date_change = todo.updated_at;
        std::thread::sleep(std::time::Duration::from_millis(1));

        todo.set_due_date(None);
        assert_eq!(todo.due_date(), None);
        assert!(todo.updated_at > updated_at_after_date_change);
    }

    #[test]
    fn test_set_priority() {
        let mut todo = create_test_todo();
        let original_updated_at = todo.updated_at;

        std::thread::sleep(std::time::Duration::from_millis(1));

        todo.set_priority(Some(Priority::Low));
        assert_eq!(todo.priority(), Some(Priority::Low));
        assert!(todo.updated_at > original_updated_at);

        let updated_at_after_priority_change = todo.updated_at;
        std::thread::sleep(std::time::Duration::from_millis(1));

        todo.set_priority(None);
        assert_eq!(todo.priority(), None);
        assert!(todo.updated_at > updated_at_after_priority_change);
    }

    #[test]
    fn test_set_tags() {
        let mut todo = create_test_todo();
        let original_updated_at = todo.updated_at;

        std::thread::sleep(std::time::Duration::from_millis(1));

        let new_tags = vec!["personal".to_string(), "health".to_string()];
        todo.set_tags(Some(new_tags.clone()));
        assert_eq!(todo.tags(), Some(new_tags.as_slice()));
        assert!(todo.updated_at > original_updated_at);

        let updated_at_after_tags_change = todo.updated_at;
        std::thread::sleep(std::time::Duration::from_millis(1));

        todo.set_tags(None);
        assert_eq!(todo.tags(), None);
        assert!(todo.updated_at > updated_at_after_tags_change);
    }

    #[test]
    fn test_priority_enum() {
        assert_eq!(Priority::High.to_string(), "🔴 High");
        assert_eq!(Priority::Medium.to_string(), "🟡 Medium");
        assert_eq!(Priority::Low.to_string(), "🟢 Low");
    }

    #[test]
    fn test_priority_from_str() {
        assert_eq!("high".parse::<Priority>().unwrap(), Priority::High);
        assert_eq!("HIGH".parse::<Priority>().unwrap(), Priority::High);
        assert_eq!("High".parse::<Priority>().unwrap(), Priority::High);

        assert_eq!("medium".parse::<Priority>().unwrap(), Priority::Medium);
        assert_eq!("MEDIUM".parse::<Priority>().unwrap(), Priority::Medium);
        assert_eq!("Medium".parse::<Priority>().unwrap(), Priority::Medium);

        assert_eq!("low".parse::<Priority>().unwrap(), Priority::Low);
        assert_eq!("LOW".parse::<Priority>().unwrap(), Priority::Low);
        assert_eq!("Low".parse::<Priority>().unwrap(), Priority::Low);

        assert!("invalid".parse::<Priority>().is_err());
        assert!("".parse::<Priority>().is_err());
    }

    #[test]
    fn test_priority_ordering() {
        assert!(Priority::High > Priority::Medium);
        assert!(Priority::Medium > Priority::Low);
        assert!(Priority::High > Priority::Low);

        assert!(Some(Priority::High) > Some(Priority::Medium));
        assert!(Some(Priority::Medium) > Some(Priority::Low));
        assert!(Some(Priority::Low) > None);
    }

    #[test]
    fn test_sort_by_enum() {
        assert_eq!("due".parse::<SortBy>().unwrap(), SortBy::Due);
        assert_eq!("DUE".parse::<SortBy>().unwrap(), SortBy::Due);
        assert_eq!("priority".parse::<SortBy>().unwrap(), SortBy::Priority);
        assert_eq!("PRIORITY".parse::<SortBy>().unwrap(), SortBy::Priority);
        assert_eq!(
            "due+priority".parse::<SortBy>().unwrap(),
            SortBy::DueThenPriority
        );
        assert_eq!(
            "DUE+PRIORITY".parse::<SortBy>().unwrap(),
            SortBy::DueThenPriority
        );

        let result: Result<SortBy, TodoError> = "invalid".parse();
        assert!(result.is_err());
        if let Err(TodoError::InvalidSortField { field }) = result {
            assert_eq!(field, "invalid");
        } else {
            panic!("Expected InvalidSortField error");
        }
    }

    #[test]
    fn test_serialization() {
        let todo = create_test_todo();

        let json = serde_json::to_string(&todo).expect("Should serialize");
        assert!(json.contains("Test Todo"));
        assert!(json.contains("Test description"));
        assert!(json.contains("High"));

        let deserialized: TodoItem = serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(deserialized.id(), todo.id());
        assert_eq!(deserialized.title(), todo.title());
        assert_eq!(deserialized.description(), todo.description());
        assert_eq!(deserialized.completed(), todo.completed());
        assert_eq!(deserialized.due_date(), todo.due_date());
        assert_eq!(deserialized.priority(), todo.priority());
        assert_eq!(deserialized.tags(), todo.tags());
    }

    #[test]
    fn test_clone() {
        let original = create_test_todo();
        let cloned = original.clone();

        assert_eq!(original.id(), cloned.id());
        assert_eq!(original.title(), cloned.title());
        assert_eq!(original.description(), cloned.description());
        assert_eq!(original.completed(), cloned.completed());
        assert_eq!(original.due_date(), cloned.due_date());
        assert_eq!(original.priority(), cloned.priority());
        assert_eq!(original.tags(), cloned.tags());
    }

    #[test]
    fn test_edge_cases() {
        let edge_todo = TodoItem::new(
            u32::MAX,
            String::new(),
            Some(String::new()),
            true,
            None,
            None,
            Some(vec![]),
        );

        assert_eq!(edge_todo.id(), u32::MAX);
        assert_eq!(edge_todo.title(), "");
        assert_eq!(edge_todo.description(), Some(""));
        assert!(edge_todo.completed());
        assert_eq!(edge_todo.tags(), Some([].as_slice()));
        assert_eq!(edge_todo.tags_string(), Some("".to_string()));
    }

    #[test]
    fn test_overdue_edge_cases() {
        let now = Local::now().naive_local();

        let todo_now = TodoItem::new(1, "Now".to_string(), None, false, Some(now), None, None);
        assert!(!todo_now.is_overdue());

        let five_minutes_ago = now - chrono::Duration::minutes(5);
        let todo_past = TodoItem::new(
            1,
            "Past".to_string(),
            None,
            false,
            Some(five_minutes_ago),
            None,
            None,
        );
        assert!(todo_past.is_overdue());

        let one_second_future = now + chrono::Duration::seconds(1);
        let todo_future = TodoItem::new(
            1,
            "Future".to_string(),
            None,
            false,
            Some(one_second_future),
            None,
            None,
        );
        assert!(!todo_future.is_overdue());
    }

    #[test]
    fn test_multiple_updates_timestamp() {
        let mut todo = create_minimal_todo();
        let original_time = todo.updated_at;

        std::thread::sleep(std::time::Duration::from_millis(2));
        todo.set_title("Updated Title".to_string());
        let after_title = todo.updated_at;
        assert!(after_title > original_time);

        std::thread::sleep(std::time::Duration::from_millis(2));
        todo.set_completed(true);
        let after_completion = todo.updated_at;
        assert!(after_completion > after_title);

        std::thread::sleep(std::time::Duration::from_millis(2));
        todo.set_priority(Some(Priority::Medium));
        let after_priority = todo.updated_at;
        assert!(after_priority > after_completion);

        assert_eq!(todo.created_at, original_time);
    }

    #[test]
    fn test_tags_with_special_characters() {
        let special_tags = vec![
            "tag with spaces".to_string(),
            "tag-with-dashes".to_string(),
            "tag_with_underscores".to_string(),
            "🏷️emoji-tag".to_string(),
            "123numeric".to_string(),
        ];

        let todo = TodoItem::new(
            1,
            "Special Tags Todo".to_string(),
            None,
            false,
            None,
            None,
            Some(special_tags.clone()),
        );

        assert_eq!(todo.tags(), Some(special_tags.as_slice()));
        assert_eq!(
            todo.tags_string(),
            Some(
                "tag with spaces, tag-with-dashes, tag_with_underscores, 🏷️emoji-tag, 123numeric"
                    .to_string()
            )
        );
    }
}
