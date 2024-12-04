//! Utilities and helper tools used internally for testing adversary.

#![cfg(test)]

use crate::Canonical;

pub fn check_impls_canonical<T: Canonical>() {}
