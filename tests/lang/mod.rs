/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Language Module Tests
//!
//! Tests for language-related utility functions.

// Argument validation module tests
mod argument {
    pub(crate) mod collection_tests;
    pub(crate) mod condition_tests;
    pub(crate) mod error_tests;
    pub(crate) mod numeric_tests;
    pub(crate) mod option_tests;
    pub(crate) mod string_tests;
}

// Data type tests
mod data_type_tests;
