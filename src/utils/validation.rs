use crate::todo::{TodoError, TodoResult};

pub fn validate_text(text: &str, len: usize) -> TodoResult<String> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Err(TodoError::EmptyTodo);
    }
    if trimmed.len() > len {
        return Err(TodoError::TodoTooLong {
            actual: trimmed.len(),
            max: len,
        });
    }
    Ok(trimmed.to_string())
}

pub fn validate_id(id_str: &str) -> TodoResult<u32> {
    if id_str.trim().is_empty() {
        return Err(TodoError::InvalidId {
            id: "empty".to_string(),
        });
    }
    if id_str.trim() == "0" {
        return Err(TodoError::InvalidId {
            id: "zero".to_string(),
        });
    }
    id_str
        .trim()
        .parse::<u32>()
        .map_err(|_| TodoError::InvalidId {
            id: id_str.to_string(),
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::todo::TodoError;

    #[cfg(test)]
    mod validate_text_tests {
        use super::*;

        #[test]
        fn test_valid_text() {
            let result = validate_text("Hello world", 20);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), "Hello world");
        }

        #[test]
        fn test_valid_text_with_whitespace() {
            let result = validate_text("  Hello world  ", 20);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), "Hello world");
        }

        #[test]
        fn test_text_at_exact_length_limit() {
            let text = "a".repeat(10);
            let result = validate_text(&text, 10);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), text);
        }

        #[test]
        fn test_empty_text() {
            let result = validate_text("", 10);
            assert!(result.is_err());
            assert!(matches!(result.unwrap_err(), TodoError::EmptyTodo));
        }

        #[test]
        fn test_whitespace_only_text() {
            let result = validate_text("   \t\n  ", 10);
            assert!(result.is_err());
            assert!(matches!(result.unwrap_err(), TodoError::EmptyTodo));
        }

        #[test]
        fn test_text_too_long() {
            let long_text = "a".repeat(15);
            let result = validate_text(&long_text, 10);
            assert!(result.is_err());
            match result.unwrap_err() {
                TodoError::TodoTooLong { actual, max } => {
                    assert_eq!(actual, 15);
                    assert_eq!(max, 10);
                }
                _ => panic!("Expected TodoTooLong error"),
            }
        }

        #[test]
        fn test_text_too_long_after_trimming() {
            let text = format!("  {}  ", "a".repeat(15));
            let result = validate_text(&text, 10);
            assert!(result.is_err());
            match result.unwrap_err() {
                TodoError::TodoTooLong { actual, max } => {
                    assert_eq!(actual, 15);
                    assert_eq!(max, 10);
                }
                _ => panic!("Expected TodoTooLong error"),
            }
        }

        #[test]
        fn test_zero_length_limit() {
            let result = validate_text("a", 0);
            assert!(result.is_err());
            match result.unwrap_err() {
                TodoError::TodoTooLong { actual, max } => {
                    assert_eq!(actual, 1);
                    assert_eq!(max, 0);
                }
                _ => panic!("Expected TodoTooLong error"),
            }
        }
    }

    #[cfg(test)]
    mod validate_id_tests {
        use super::*;

        #[test]
        fn test_valid_positive_id() {
            let result = validate_id("123");
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), 123);
        }

        #[test]
        fn test_valid_id_with_whitespace() {
            let result = validate_id("  456  ");
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), 456);
        }

        #[test]
        fn test_valid_id_one() {
            let result = validate_id("1");
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), 1);
        }

        #[test]
        fn test_valid_large_id() {
            let result = validate_id("4294967295");
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), 4294967295);
        }

        #[test]
        fn test_empty_id() {
            let result = validate_id("");
            assert!(result.is_err());
            match result.unwrap_err() {
                TodoError::InvalidId { id } => {
                    assert_eq!(id, "empty");
                }
                _ => panic!("Expected InvalidId error with 'empty'"),
            }
        }

        #[test]
        fn test_whitespace_only_id() {
            let result = validate_id("   \t\n  ");
            assert!(result.is_err());
            match result.unwrap_err() {
                TodoError::InvalidId { id } => {
                    assert_eq!(id, "empty");
                }
                _ => panic!("Expected InvalidId error with 'empty'"),
            }
        }

        #[test]
        fn test_zero_id() {
            let result = validate_id("0");
            assert!(result.is_err());
            match result.unwrap_err() {
                TodoError::InvalidId { id } => {
                    assert_eq!(id, "zero");
                }
                _ => panic!("Expected InvalidId error with 'zero'"),
            }
        }

        #[test]
        fn test_zero_id_with_whitespace() {
            let result = validate_id("  0  ");
            assert!(result.is_err());
            match result.unwrap_err() {
                TodoError::InvalidId { id } => {
                    assert_eq!(id, "zero");
                }
                _ => panic!("Expected InvalidId error with 'zero'"),
            }
        }

        #[test]
        fn test_negative_id() {
            let result = validate_id("-1");
            assert!(result.is_err());
            match result.unwrap_err() {
                TodoError::InvalidId { id } => {
                    assert_eq!(id, "-1");
                }
                _ => panic!("Expected InvalidId error"),
            }
        }

        #[test]
        fn test_non_numeric_id() {
            let result = validate_id("abc");
            assert!(result.is_err());
            match result.unwrap_err() {
                TodoError::InvalidId { id } => {
                    assert_eq!(id, "abc");
                }
                _ => panic!("Expected InvalidId error"),
            }
        }

        #[test]
        fn test_mixed_alphanumeric_id() {
            let result = validate_id("123abc");
            assert!(result.is_err());
            match result.unwrap_err() {
                TodoError::InvalidId { id } => {
                    assert_eq!(id, "123abc");
                }
                _ => panic!("Expected InvalidId error"),
            }
        }

        #[test]
        fn test_float_id() {
            let result = validate_id("12.34");
            assert!(result.is_err());
            match result.unwrap_err() {
                TodoError::InvalidId { id } => {
                    assert_eq!(id, "12.34");
                }
                _ => panic!("Expected InvalidId error"),
            }
        }

        #[test]
        fn test_id_too_large_for_u32() {
            let result = validate_id("4294967296");
            assert!(result.is_err());
            match result.unwrap_err() {
                TodoError::InvalidId { id } => {
                    assert_eq!(id, "4294967296");
                }
                _ => panic!("Expected InvalidId error"),
            }
        }
    }
}
