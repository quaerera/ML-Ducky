use num_traits::float::Float;
use num_traits::int::PrimInt;

use crate::cell::Cell;
use crate::series::Series;

use std::f64;
use num_traits::FromPrimitive;

fn sigmoid(x: f64) -> f64 {
    x.signum()
}

fn minmax_item<T: Into<f64> + Copy>(min_max: (f64, f64), item: T) -> (f64, f64) {
    (
        min_max.0.min(item.into()),
        min_max.1.max(item.into())
    )
}

fn minmax<T: Into<f64> + Copy>(series: Vec<T>) -> Option<(f64, f64)> {
    if series.is_empty() {
        ()
    }
    let mut min_max = (series[0].into(), series[0].into());
    series
        .iter()
        .skip(1)
        .for_each(
            | &element| {
                min_max = minmax_item(min_max, element);
            });
    Some(min_max)
}

fn series_minmax(series: Series<Cell>) -> (f64, f64) {
    let mut min_max: (f64, f64) = (f64::nan(), f64::nan());
    for element in series.data {
        match element {
            Cell::Integer(cell) => {
                min_max = minmax_item(min_max, cell);
            },
            Cell::Float(cell) => {
                min_max = minmax_item(min_max, cell);
            },
            Cell::Float64(cell) => {
                min_max = minmax_item(min_max, cell);
            },
            _ => {}
        }
    }
    min_max
}

#[cfg(test