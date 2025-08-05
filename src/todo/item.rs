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

    pub fn set_title(&mut self, title: String) {
        self.title = title;
        self.updated_at = Local::now().naive_local();
    }

    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
        self.updated_at = Local::now().naive_local();
    }

    pub fn set_completed(&mut self, completed: bool) {
        self.completed = completed;
        self.updated_at = Local::now().naive_local();
    }

    pub fn set_due_date(&mut self, due_date: Option<NaiveDateTime>) {
        self.due_date = due_date;
        self.updated_at = Local::now().naive_local();
    }

    pub fn set_priority(&mut self, priority: Option<Priority>) {
        self.priority = priority;
        self.updated_at = Local::now().naive_local();
    }

    pub fn set_tags(&mut self, tags: Option<Vec<String>>) {
        self.tags = tags;
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
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "high" => Ok(Priority::High),
            "medium" => Ok(Priority::Medium),
            "low" => Ok(Priority::Low),
            _ => Err(()),
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
