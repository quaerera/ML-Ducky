use std::iter::Product;

use num::{PrimInt, Signed};

use crate::dataframe::DataFrame;
use crate::algebraic::matrix::Matrix;
use crate::cell::Cell;

pub trait Science {
    /// One hot encoding - Convert string values to binary value
    fn get_dummies(&mut self, label: