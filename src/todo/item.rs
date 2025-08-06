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
        assert!(todo.due_date().is_none());
        assert_eq!(todo.priority(), None);
        assert_eq!(todo.tags(), None);
    }

    #[test]
    fn test_setter_updates_timestamp() {
        let mut todo = create_test_todo();
        let original_updated_at = todo.updated_at;

        std::thread::sleep(std::time::Duration::from_millis(1));

        todo.set_title("New Title".to_string());

        assert_eq!(todo.title(), "New Title");
        assert!(todo.updated_at > original_updated_at);
        assert_eq!(todo.created_at, original_updated_at);
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
    fn test_priority_from_str() {
        assert_eq!("high".parse::<Priority>().unwrap(), Priority::High);
        assert_eq!("MEDIUM".parse::<Priority>().unwrap(), Priority::Medium);
        assert_eq!("Low".parse::<Priority>().unwrap(), Priority::Low);

        assert!("invalid".parse::<Priority>().is_err());
        assert!("".parse::<Priority>().is_err());
    }

    #[test]
    fn test_sort_by_from_str() {
        assert_eq!("due".parse::<SortBy>().unwrap(), SortBy::Due);
        assert_eq!("PRIORITY".parse::<SortBy>().unwrap(), SortBy::Priority);
        assert_eq!(
            "due+priority".parse::<SortBy>().unwrap(),
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

        let deserialized: TodoItem = serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(deserialized.id(), todo.id());
        assert_eq!(deserialized.title(), todo.title());
        assert_eq!(deserialized.description(), todo.description());
        assert_eq!(deserialized.completed(), todo.completed());
        assert_eq!(deserialized.due_date(), todo.due_date());
        assert_eq!(deserialized.priority(), todo.priority());
        assert_eq!(deserialized.tags(), todo.tags());
    }
}
