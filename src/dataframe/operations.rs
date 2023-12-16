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
    fn to_rows(&self) -> Option<Vec<Vec<Cell>>>;

    /// Get selected column by using label name
    fn by(&mut self, label: &str) -> Option<&mut Series<Cell>>;

    /// Get selected column by using label name
    fn many(&mut self, labels: Vec<&str>) -> Vec<Series<Cell>>;

    fn map(&mut self, col: &str, obj: HashMap<&str, u32>) -> DataFra