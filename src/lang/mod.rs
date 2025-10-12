/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Language Tools
//!
//! Provides common language-level tools and utility functions.
//!
//! # Author
//!
//! Hu Haixing

pub mod argument;
pub mod data_type;

pub use data_type::{DataType, DataTypeOf};

// Re-export commonly used types
pub use argument::{
    check_argument, check_state, ArgumentError, ArgumentResult, CollectionArgument,
    NumericArgument, OptionArgument, StringArgument,
};
