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
    fn new(vec: Vec<Vec<Cell>>, labels: Vec<&str>) -> DataFrame {
        let mut column_types = vec![];
        // Figure out the column types from the data
        for i in 0..vec[0].len() {
            column_types.push(vec[0][i].data_type());
        }

        // create columns based on column types
        let mut cols = Vec::<Series<Cell>>::new();
        for (i, t) in column_types.iter().enumerate() {
            match t {
                DataTypes::Text => cols.push(Series {
                    label: labels[i].to_string(),
                    data: Vec::new(),
                }),
                DataTypes::Float => cols.push(Series {
                    label: labels[i].to_string(),
                    data: Vec::new(),
                }),
                _ => cols.push(Series {
                    label: labels[i].to_string(),
                    data: Vec::new(),
                }),
            }
        }

        let mut size = 0;
        for row in vec.iter() {
            for (col_index, cell) in row.iter().enumerate() {
                match cell {
                    Cell::Float(c) => {
                        cols[col_index].data.push(cell.clone());
                    }
                    _ => {
                        cols[col_index].data.push(cell.clone());
                    }
                }
            }
            size += 1;
        }
        let mut tmp: Vec<String> = Vec::new();
        for label in labels {
            tmp.push(label.to_string())
        }
        DataFrame {
            size,
            labels: tmp,
            data: cols,
        }
    }

    fn push(&mut self, vec: Vec<Cell>) {
        for (i, el) in vec.iter().enumerate() {
            self.data[i].data.push(el.clone());
        }
        self.size += 1;
    }

    fn series(&mut self, index: usize) -> &mut Series<Cell> {
        &mut self.data[index]
    }

    fn by(&mut self, label: &str) -> Option<&mut Series<Cell>> {
        let index = self.labels
            .clone()
            .iter()
            .position(
                |el| el == label
            )
            .unwrap();
        Some(self.series((index).to_owned()))
    }

    fn many(&mut self, labels: Vec<&str>) -> Vec<Series<Cell>> {
        let mut vec: Vec<Series<Cell>> = V