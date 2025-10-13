/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Prism3 Core - Core Utility Library
//!
//! Provides language-level fundamental tools and data type support, including:
//! - Data type definitions and validation
//! - Argument validation and error handling
//! - Core utility functions
//!
//! # Author
//!
//! Hu Haixing

pub mod lang;
pub mod util;

// Re-export main types from lang module
pub use lang::{
    argument::{
        // Core functions
        check_argument,
        // Condition functions
        check_argument_fmt,
        check_argument_with_message,
        check_bounds,
        check_element_index,
        check_position_index,
        check_position_indexes,
        check_state,
        check_state_with_message,
        // Collection functions
        require_element_non_null,
        // Numeric functions
        require_equal,
        require_not_equal,
        // Option functions
        require_null_or,
        ArgumentError,
        ArgumentResult,
        CollectionArgument,
        NumericArgument,
        OptionArgument,
        // String functions
        StringArgument,
    },
    data_type::{DataType, DataTypeOf},
};

// Re-export utility types
pub use util::{Pair, Triple};
