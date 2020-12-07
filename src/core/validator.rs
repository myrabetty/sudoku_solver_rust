//``` contains a function to validate the grid checking no duplicate values are present in quadrants,
// rows and columns.
use ndarray::Array2;

use crate::core::model::{EmptyCell, NonEmptyCell, GridFunctions};

//validate there are no duplications with the values set in the grid.
pub fn validate_grid(grid: &Array2<NonEmptyCell>) -> Result<(), String> {
    for value in 1..10 {
        for j in 0..9 {
            let result_row = validate_row(grid, value, j);
            if result_row.is_err() {
                return result_row;
            }

            let result_column = validate_column(grid, value, j);
            if result_column.is_err() {
                return result_column;
            }

            let result_quadrant = validate_quadrant(grid, value, j);
            if result_quadrant.is_err() {
                return result_quadrant;
            }
        }
    }
    Ok(())
}

fn validate_quadrant(grid: &Array2<NonEmptyCell>, value: u8, j: usize) -> Result<(), String> {
    let count = grid.iter().filter(|&x| x.quadrant == j && x.value == value).count();
    if count > 1 {
        return Err(format!("{} duplicates in quadrant {} for number {}", count, j, value));
    }
    return Ok(());
}

fn validate_column(grid: &Array2<NonEmptyCell>, value: u8, j: usize) -> Result<(), String> {
    let count = grid.column(j).iter().filter(|&x| x.value == value).count();
    if count > 1 {
        return Err(format!("{} duplicates in column {} for number {}", count, j, value));
    }
    return Ok(());
}

fn validate_row(grid: &Array2<NonEmptyCell>, value: u8, j: usize) -> Result<(), String> {
    let count = grid.row(j).iter().filter(|&x| x.value == value).count();
    if count > 1 {
        return Err(format!("{} duplicates in row {} for number {}", count, j, value));
    }
    return Ok(());
}

trait GetValues {
    fn get_values(&self) -> Vec<u8>;
}

impl GetValues for Vec<NonEmptyCell> {
    fn get_values(&self) -> Vec<u8> {
        self.iter().map(|x| x.value).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::model::{NonEmptyCell, GridFunctions};

    #[test]
    fn validate_row_test() {
        let grid = generate_grid();
        validate_row(&grid, 1, 0).expect_err("error");
        validate_row(&grid, 2, 0).expect("no error");
    }

    #[test]
    fn validate_column_test() {
        let grid = generate_grid();
        validate_column(&grid, 9, 0).expect_err("error");
        validate_column(&grid, 2, 0).expect("no error");
    }

    #[test]
    fn validate_quadrant_test() {
        let grid = generate_grid();
        validate_quadrant(&grid, 9, 0).expect_err("error");
        validate_quadrant(&grid, 1, 0).expect_err("error");
        validate_quadrant(&grid, 3, 0).expect("no error");
    }

    #[test]
    fn validate_grid_test() {
        let grid = generate_grid();
        validate_grid(&grid).expect_err("error result");
    }

    fn generate_grid() -> Array2<NonEmptyCell> {
        let mut grid: Array2<NonEmptyCell> = Array2::default((9, 9));
        grid.add_quadrants_information();
        grid.column_mut(0).iter_mut().for_each(|mut x| x.value = 9);
        grid.row_mut(0).iter_mut().for_each(|mut x| x.value = 1);
        return grid;
    }
}
