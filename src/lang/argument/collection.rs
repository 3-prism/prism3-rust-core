/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Collection Argument Validation
//!
//! Provides validation functionality for collection type arguments.
//!
//! # Author
//!
//! Hu Haixing

use super::error::{ArgumentError, ArgumentResult};

/// # Collection Argument Validation Trait
///
/// Provides length and content validation functionality for collection types like slices, Vec, arrays, etc.
///
/// # Features
///
/// - Non-empty checking support
/// - Length validation support
/// - Element validation support
/// - Method chaining support
///
/// # Use Cases
///
/// - Validating function parameter collections
/// - Configuration array checking
/// - User input list validation
///
/// # Examples
///
/// Basic usage (returns `ArgumentResult`):
///
/// ```rust,ignore
/// use prism3_core::lang::argument::{CollectionArgument, ArgumentResult};
///
/// fn process_items(items: &[String]) -> ArgumentResult<()> {
///     let items = items.require_non_empty("items")?;
///     println!("Processing {} items", items.len());
///     Ok(())
/// }
/// ```
///
/// Converting to other error types:
///
/// ```rust,ignore
/// use prism3_core::lang::argument::CollectionArgument;
///
/// fn process_items(items: &[String]) -> Result<(), String> {
///     let items = items
///         .require_non_empty("items")
///         .and_then(|i| i.require_length_in_range("items", 1, 100))
///         .map_err(|e| e.to_string())?;
///     println!("Processing {} items", items.len());
///     Ok(())
/// }
/// ```
///
/// # Author
///
/// Hu Haixing
///
pub trait CollectionArgument {
    /// Validate that the collection is not empty
    ///
    /// # Parameters
    ///
    /// * `name` - Parameter name
    ///
    /// # Returns
    ///
    /// Returns `Ok(self)` if the collection is not empty, otherwise returns an error
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use prism3_core::lang::argument::CollectionArgument;
    ///
    /// let items = vec![1, 2, 3];
    /// assert!(items.require_non_empty("items").is_ok());
    ///
    /// let empty: Vec<i32> = vec![];
    /// assert!(empty.require_non_empty("items").is_err());
    /// ```
    fn require_non_empty(&self, name: &str) -> ArgumentResult<&Self>;

    /// Validate that the collection length equals the specified value
    ///
    /// # Parameters
    ///
    /// * `name` - Parameter name
    /// * `length` - Expected length
    ///
    /// # Returns
    ///
    /// Returns `Ok(self)` if the length matches, otherwise returns an error
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use prism3_core::lang::argument::CollectionArgument;
    ///
    /// let coordinates = vec![1, 2, 3];
    /// assert!(coordinates.require_length_be("coordinates", 3).is_ok());
    /// ```
    fn require_length_be(&self, name: &str, length: usize) -> ArgumentResult<&Self>;

    /// Validate that the collection length is at least the specified value
    ///
    /// # Parameters
    ///
    /// * `name` - Parameter name
    /// * `min_length` - Minimum length
    ///
    /// # Returns
    ///
    /// Returns `Ok(self)` if the length is not less than the minimum, otherwise returns an error
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use prism3_core::lang::argument::CollectionArgument;
    ///
    /// let items = vec![1, 2, 3, 4, 5];
    /// assert!(items.require_length_at_least("items", 3).is_ok());
    /// ```
    fn require_length_at_least(&self, name: &str, min_length: usize) -> ArgumentResult<&Self>;

    /// Validate that the collection length is at most the specified value
    ///
    /// # Parameters
    ///
    /// * `name` - Parameter name
    /// * `max_length` - Maximum length
    ///
    /// # Returns
    ///
    /// Returns `Ok(self)` if the length is not greater than the maximum, otherwise returns an error
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use prism3_core::lang::argument::CollectionArgument;
    ///
    /// let items = vec![1, 2, 3];
    /// assert!(items.require_length_at_most("items", 10).is_ok());
    /// ```
    fn require_length_at_most(&self, name: &str, max_length: usize) -> ArgumentResult<&Self>;

    /// Validate that the collection length is within the specified range
    ///
    /// # Parameters
    ///
    /// * `name` - Parameter name
    /// * `min_length` - Minimum length (inclusive)
    /// * `max_length` - Maximum length (inclusive)
    ///
    /// # Returns
    ///
    /// Returns `Ok(self)` if the length is within range, otherwise returns an error
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use prism3_core::lang::argument::CollectionArgument;
    ///
    /// let items = vec![1, 2, 3];
    /// assert!(items.require_length_in_range("items", 1, 10).is_ok());
    /// ```
    fn require_length_in_range(
        &self,
        name: &str,
        min_length: usize,
        max_length: usize,
    ) -> ArgumentResult<&Self>;
}

impl<T> CollectionArgument for [T] {
    fn require_non_empty(&self, name: &str) -> ArgumentResult<&Self> {
        if self.is_empty() {
            return Err(ArgumentError::new(format!("Collection '{}' cannot be empty", name)));
        }
        Ok(self)
    }

    fn require_length_be(&self, name: &str, length: usize) -> ArgumentResult<&Self> {
        let actual_length = self.len();
        if actual_length != length {
            return Err(ArgumentError::new(format!(
                "Collection '{}' length must be {} but was {}",
                name, length, actual_length
            )));
        }
        Ok(self)
    }

    fn require_length_at_least(&self, name: &str, min_length: usize) -> ArgumentResult<&Self> {
        let actual_length = self.len();
        if actual_length < min_length {
            return Err(ArgumentError::new(format!(
                "Collection '{}' length must be at least {} but was {}",
                name, min_length, actual_length
            )));
        }
        Ok(self)
    }

    fn require_length_at_most(&self, name: &str, max_length: usize) -> ArgumentResult<&Self> {
        let actual_length = self.len();
        if actual_length > max_length {
            return Err(ArgumentError::new(format!(
                "Collection '{}' length must be at most {} but was {}",
                name, max_length, actual_length
            )));
        }
        Ok(self)
    }

    fn require_length_in_range(
        &self,
        name: &str,
        min_length: usize,
        max_length: usize,
    ) -> ArgumentResult<&Self> {
        let actual_length = self.len();
        if actual_length < min_length || actual_length > max_length {
            return Err(ArgumentError::new(format!(
                "Collection '{}' length must be in range [{}, {}] but was {}",
                name, min_length, max_length, actual_length
            )));
        }
        Ok(self)
    }
}

impl<T> CollectionArgument for Vec<T> {
    fn require_non_empty(&self, name: &str) -> ArgumentResult<&Self> {
        if self.is_empty() {
            return Err(ArgumentError::new(format!("Collection '{}' cannot be empty", name)));
        }
        Ok(self)
    }

    fn require_length_be(&self, name: &str, length: usize) -> ArgumentResult<&Self> {
        let actual_length = self.len();
        if actual_length != length {
            return Err(ArgumentError::new(format!(
                "Collection '{}' length must be {} but was {}",
                name, length, actual_length
            )));
        }
        Ok(self)
    }

    fn require_length_at_least(&self, name: &str, min_length: usize) -> ArgumentResult<&Self> {
        let actual_length = self.len();
        if actual_length < min_length {
            return Err(ArgumentError::new(format!(
                "Collection '{}' length must be at least {} but was {}",
                name, min_length, actual_length
            )));
        }
        Ok(self)
    }

    fn require_length_at_most(&self, name: &str, max_length: usize) -> ArgumentResult<&Self> {
        let actual_length = self.len();
        if actual_length > max_length {
            return Err(ArgumentError::new(format!(
                "Collection '{}' length must be at most {} but was {}",
                name, max_length, actual_length
            )));
        }
        Ok(self)
    }

    fn require_length_in_range(
        &self,
        name: &str,
        min_length: usize,
        max_length: usize,
    ) -> ArgumentResult<&Self> {
        let actual_length = self.len();
        if actual_length < min_length || actual_length > max_length {
            return Err(ArgumentError::new(format!(
                "Collection '{}' length must be in range [{}, {}] but was {}",
                name, min_length, max_length, actual_length
            )));
        }
        Ok(self)
    }
}

/// Validate that all elements in the collection are non-null
///
/// Checks a collection of Option types to ensure all elements are Some.
///
/// # Parameters
///
/// * `name` - Parameter name
/// * `collection` - Collection to validate
///
/// # Returns
///
/// Returns `Ok(())` if all elements are non-null, otherwise returns an error containing the index of the first null element
///
/// # Examples
///
/// ```rust,ignore
/// use prism3_core::lang::argument::require_element_non_null;
///
/// let items = vec![Some(1), Some(2), Some(3)];
/// assert!(require_element_non_null("items", &items).is_ok());
///
/// let items_with_none = vec![Some(1), None, Some(3)];
/// assert!(require_element_non_null("items", &items_with_none).is_err());
/// ```
///
/// # Author
///
/// Hu Haixing
///
pub fn require_element_non_null<T>(name: &str, collection: &[Option<T>]) -> ArgumentResult<()> {
    for (index, item) in collection.iter().enumerate() {
        if item.is_none() {
            return Err(ArgumentError::new(format!(
                "Collection '{}': element at index {} cannot be null",
                name, index
            )));
        }
    }
    Ok(())
}
