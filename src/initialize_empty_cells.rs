pub(crate) mod initialize_empty_cells {
    use ndarray::Array2;

    use crate::model::model::{EmptyCell, EmptyCellFunctions, GridFunctions, NonEmptyCell};

    // ```given a grid returns for each empty cell the possible values.
    pub fn set_allowed_values(grid: &Array2<NonEmptyCell>) -> Vec<EmptyCell> {
        let mut guesses: Vec<EmptyCell> = Vec::new();
        for i in 0..9 {
            for j in 0..9 {
                if grid[[i, j]].value == 0 {
                    let guess = EmptyCell::create(i, j);
                    guesses.push(guess);
                }
            }
        }

        guesses.iter_mut().for_each(|mut guess| {
            let existing_values = get_existing_values(grid, guess);
            eliminate_existing_values(existing_values, &mut guess);
            //if guess.values.is_empty() { errors.push(format!("Guess for empty cell i={}, j ={} is null", guess.row, guess.column)) }
        });
        return guesses;
        /* if errors.is_empty() {
            return Ok(guesses);
        }
        return Err(errors); */
    }

    fn eliminate_existing_values(existing_values: Vec<u8>, guess: &mut EmptyCell) {
        guess.values.retain(|x| !existing_values.contains(x));
    }

    fn get_existing_values(grid: &Array2<NonEmptyCell>, guess: &EmptyCell) -> Vec<u8> {
        let mut all_values = grid.row(guess.row).map(|&x| x.value).to_vec();

        let mut columns_values: Vec<u8> = grid.column(guess.row).iter()
            .map(|&x| x.value)
            .filter(|x| all_values.contains(x)).collect();

        all_values.append(&mut columns_values);

        let mut quadrant_values = grid.iter()
            .filter(|&x| x.quadrant == guess.quadrant)
            .map(|&x| x.value)
            .filter(|x| all_values.contains(x)).collect();

        all_values.append(&mut quadrant_values);

        all_values
    }

    #[cfg(test)]
    mod tests {
        use crate::model::model::{get_quadrant_position, GuessCell};

        use super::*;

        #[test]
        fn get_existing_values_test() {
            let grid = generate_grid();
            let mut guess = generate_empty_cell(0, 0, None);

            let existing_values = get_existing_values(&grid, &guess);
            eliminate_existing_values(existing_values, &mut guess);
            assert_eq!(vec![9], guess.values);
        }

        fn generate_empty_cell(row: usize, column: usize, values: Option<Vec<u8>>) -> EmptyCell {
            let cell = EmptyCell {
                row,
                column,
                quadrant: get_quadrant_position(row, column),
                values: values.unwrap_or(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]),
            };
            return cell;
        }

        fn generate_grid() -> Array2<NonEmptyCell> {
            let mut grid: Array2<NonEmptyCell> = Array2::default((9, 9));
            grid.add_quadrants_information();

            // populate the first row with 0,1,2,3,4,5,6,6,7,8
            grid.row_mut(0).indexed_iter_mut()
                .for_each(|(i, mut x)| x.value = i as u8);

            // populate the first column with 0,8,7,6,5,4,3,2,1
            grid.column_mut(0).indexed_iter_mut()
                .for_each(|(i, mut x)| {
                    match i {
                        0 => x.value = 0 as u8,
                        _ => x.value = 9 - i as u8
                    }
                }
                );
            return grid;
        }

        #[test]
        fn set_allowed_values_test() {
            let mut grid: Array2<NonEmptyCell> = Array2::default((9, 9));
            let grid = generate_grid();
            let guesses = set_allowed_values(&grid);

            assert_eq!(guesses.len(), 65);
            assert!(guesses.contains(&EmptyCell::new(0, 0, [9].to_vec())));
            assert!(guesses.contains(&EmptyCell::new(1, 1, [2, 3, 4, 5, 6, 7, 9].to_vec())));
            assert!(guesses.contains(&EmptyCell::new(1, 2, [1, 3, 4, 5, 6, 7, 9].to_vec())));
            assert!(guesses.contains(&EmptyCell::new(2, 1, [2, 3, 4, 5, 6, 8, 9].to_vec())));
            assert!(guesses.contains(&EmptyCell::new(2, 2, [1, 3, 4, 5, 6, 8, 9].to_vec())));
            assert!(guesses.contains(&EmptyCell::new(8, 8, [2, 3, 4, 5, 6, 7, 9].to_vec())));
        }
    }
}
