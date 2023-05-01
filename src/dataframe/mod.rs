use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::ops::{Index, IndexMut};
use std::f64;
use num::NumCast;

use crate::cell::Cell;
use crate::dataframe::operations::Operations;
use crate::dataframe::science::Science;
use crate::series::{Series, SeriesImpl};
use crate::types::DataTypes;
use std::borrow::Borrow;
use std::cmp::Ordering;
use crate::algebraic::matrix::Matrix;

pub mod science;
pub mod operations;
pub mod tests_dataframe;

#[derive(Debug, Clone)]
pub struct DataFrame {
    pub size: usize,
    pub labels: Vec<String>,
    pub data: Vec<Series<Cell>>,
}

type DataFrameGroupBy = HashMap<String, Vec<Vec<Cell>>>;

impl Operations for DataFrame {