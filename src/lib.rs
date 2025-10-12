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
    data_type::{DataType, DataTypeOf},
    argument::{
        // Core functions
        check_argument, check_state, ArgumentError, ArgumentResult,
        // Condition functions
        check_argument_fmt, check_argument_with_message, check_bounds,
        check_element_index, check_position_index, check_position_indexes, check_state_with_message,
        // Collection functions
        require_element_non_null, CollectionArgument,
        // Numeric functions
        require_equal, require_not_equal, NumericArgument,
        // Option functions
        require_null_or, OptionArgument,
        // String functions
        StringArgument,
    },
};

// Re-export utility types
pub use util::{Pair, Triple};
