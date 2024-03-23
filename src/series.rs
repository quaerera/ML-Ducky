
use std::collections::HashSet;

use crate::cell::Cell;
use crate::types::{DFloat, DInteger, DFloat64};

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Series<T> {
    pub label: String,
    pub data: Vec<T>
}