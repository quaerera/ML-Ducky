use std::collections::HashMap;
use std::error::Error;

use crate::cell::Cell;
use crate::dataframe::{DataFrame, DataFrameGroupBy};
use crate::series::Series;

pub trait Operations {
    fn new(vec: Vec<Vec<Cell>>, labels: Vec<&str>) -> DataFrame;

    fn push(&mut s