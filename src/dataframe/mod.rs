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
use std::borrow::Bor