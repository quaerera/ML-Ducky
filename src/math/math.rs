use num_traits::float::Float;
use num_traits::int::PrimInt;

use crate::cell::Cell;
use crate::series::Series;

use std::f64;
use num_traits::FromPrimitive;

fn sigmoid(x: f64) -> f64 {
    x.signum()
}

fn minmax_ite