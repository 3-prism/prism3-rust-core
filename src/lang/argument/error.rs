/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Argument Validation Errors
//!
//! Defines error types related to argument validation.
//!
//! # Author
//!
//! Haixing Hu

use std::fmt;

/// Argument validation error
///
/// Returned when an argument does not satisfy validation conditions.
///
/// # Features
///
/// - Contains detailed error messages
/// - Implements standard error traits
/// - Supports conversion from strings
///
/// # Examples
///
/// ```rust,ignore
/// use prism3_core::lang::argument::ArgumentError;
///
/// let error = ArgumentError::new("Invalid argument");
/// assert_eq!(error.message(), "Invalid argument");
/// ```
///
/// # Author
///
/// Haixing Hu
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArgumentError {
    message: String,
}

impl ArgumentError {
    /// Create a new argument error
    ///
    /// # Parameters
    ///
    /// * `message` - Error message
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use prism3_core::lang::argument::ArgumentError;
    ///
    /// let error = ArgumentError::new("Value cannot be negative");
    /// ```
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    /// Get the error message
    ///
    /// # Returns
    ///
    /// Returns a reference to the error message
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for ArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ArgumentError {}

impl From<String> for ArgumentError {
    fn from(message: String) -> Self {
        Self::new(message)
    }
}

impl From<&str> for ArgumentError {
    fn from(message: &str) -> Self {
        Self::new(message)
    }
}

/// Argument validation result type
///
/// Unified result type for all argument validation operations.
///
/// # Examples
///
/// ```rust,ignore
/// use prism3_core::lang::argument::{ArgumentResult, ArgumentError};
///
/// fn validate_positive(value: i32) -> ArgumentResult<i32> {
///     if value > 0 {
///         Ok(value)
///     } else {
///         Err(ArgumentError::new("Value must be positive"))
///     }
/// }
/// ```
///
/// # Author
///
/// Haixing Hu
///
pub type ArgumentResult<T> = Result<T, ArgumentError>;
