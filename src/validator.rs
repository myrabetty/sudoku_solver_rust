pub(crate) mod validator {
    use ndarray::Array2;

    use crate::model::model::{EmptyCell, NonEmptyCell};

    //validate there are no duplications with the values set in the grid.
    pub fn validate_grid(grid: &Array2<NonEmptyCell>) -> Result<(), String> {
        for value in 1..10 {
            for j in 0..9 {
                validate_row(grid, value, j).unwrap();
                validate_column(grid, value, j).unwrap();
                validate_quadrant(grid, value, j).unwrap();
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

    // validates the consistency of the values allowed for the empty cells by asserting that
    // none is an empty set.
    pub fn validate_allowed_values(allowed_values: Vec<EmptyCell>) -> Result<(), ()> {
        if allowed_values.iter().map(|x| x.values.is_empty()).count() > 0 {
            return Err(());
        }
        Ok(())
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
        use crate::model::model::GridFunctions;

        use super::*;

        #[test]
        fn validate_rows() {
            let grid = generate_grid();
            validate_row(&grid, 1, 0).expect_err("error");
            validate_row(&grid, 2, 0).expect("no error");
        }

        #[test]
        fn validate_columns() {
            let grid = generate_grid();
            validate_column(&grid, 9, 0).expect_err("error");
            validate_column(&grid, 2, 0).expect("no error");
        }

        #[test]
        fn validate_quadrants() {
            let grid = generate_grid();
            validate_quadrant(&grid, 9, 0).expect_err("error");
            validate_quadrant(&grid, 1, 0).expect_err("error");
            validate_quadrant(&grid, 3, 0).expect("no error");
        }

        fn generate_grid() -> Array2<NonEmptyCell> {
            let mut grid: Array2<NonEmptyCell> = Array2::default((9, 9));
            grid.add_quadrants_information();
            grid.column_mut(0).iter_mut().for_each(|mut x| x.value = 9);
            grid.row_mut(0).iter_mut().for_each(|mut x| x.value = 1);
            return grid;
        }
    }
}
