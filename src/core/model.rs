use std::collections::HashMap;

use logs::debug;
use ndarray::Array2;

use crate::core::utilities::get_quadrant;

// ```it describes a cell position and its value
#[derive(Debug, Default, Clone)]
pub struct NonEmptyCell {
    pub value: u8,
    pub quadrant: usize,
}

//```it describes an empty cell position and the values that can be in that cell.
#[derive(Debug, Default, Clone)]
pub struct EmptyCell {
    pub row: usize,
    pub column: usize,
    pub quadrant: usize,
    pub values: Vec<u8>,
}

impl EmptyCell {
    pub(crate) fn eliminate(&mut self, value: u8) {
        if let Some(index) = self.values.iter().position(|&x| x == value) {
            self.values.remove(index);
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Guess {
    pub row: usize,
    pub column: usize,
    pub value: u8,
}

pub trait GridFunctions {
    // ``` add quadrant information to a grid
    fn add_quadrants_information(&mut self);
    // ```check if the grid is complete
    fn is_complete(&self) -> bool;
}

pub trait EmptyCellFunctions {
    fn with_all_values(row: usize, column: usize) -> Self;
    fn new(row: usize, column: usize, values: Vec<u8>) -> Self;
}

impl GridFunctions for Array2<NonEmptyCell> {
    fn add_quadrants_information(&mut self) {
        for i in 0..9 {
            for j in 0..9 {
                self[[i, j]].quadrant = get_quadrant(i, j);
                debug!("row = {}, column = {}, and quadrant {}", i, j, self[[i, j]].quadrant);
            }
        }
    }

    fn is_complete(&self) -> bool {
        return self.iter().filter(|&x| x.value == 0).count() == 0;
    }
}

impl EmptyCellFunctions for EmptyCell {
    fn with_all_values(row: usize, column: usize) -> Self {
        return EmptyCell {
            row,
            column,
            quadrant: get_quadrant(row, column),
            values: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        };
    }
    fn new(row: usize, column: usize, values: Vec<u8>) -> Self {
        return EmptyCell {
            row,
            column,
            quadrant: get_quadrant(row, column),
            values,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}

