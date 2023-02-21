
use crate::types::{DataTypes, DFloat, DInteger, DFloat64};

/// Basic elementary cell in data frame
#[derive(Debug, Clone, PartialEq)]
pub enum Cell {
    Text(String),
    Integer(i32),
    Float(f32),
    Float64(f64),
    Bool(bool)
}

impl Cell {
    pub fn data_type(&self) -> DataTypes {
        match self {
            Cell::Text(_) => DataTypes::Text,
            Cell::Integer(_) => DataTypes::Integer,
            Cell::Float(_) => DataTypes::Float,
            Cell::Float64(_) => DataTypes::Float64,
            Cell::Bool(_) => DataTypes::Bool,
            _ => DataTypes::Text
        }
    }

}

impl Into<String> for Cell {
    fn into(self) -> String {
       match self {
           Cell::Text(v) => v,
           _ => {panic!("Error")}
       }
    }
}
