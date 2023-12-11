use std::collections::HashMap;
use std::error::Error;

use crate::cell::Cell;
use crate::dataframe::{DataFrame, DataFrameGroupBy};
use crate::series::Series;

pub trait Operations {
    fn new(vec: Vec<Vec<Cell>>, labels: Vec<&str>) -> DataFrame;

    fn push(&mut self, element: Vec<Cell>);

    fn series(&mut self, index: usize) -> &mut Series<Cell>;

    /// From column series to rows
    fn to_rows(&self) -> Option<Vec<Vec<Cell>