pub(crate) mod initialize_empty_cells {
    use ndarray::Array2;

    use crate::model::model::{EmptyCell, EmptyCellFunctions, GridFunctions, NonEmptyCell};
    use crate::utilities::utilities::iters_equal_any_order;

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
        });
        return guesses;
    }

    fn eliminate_existing_values(existing_values: Vec<u8>, guess: &mut EmptyCell) {
        guess.values.retain(|x| !existing_values.contains(x));
    }

    fn get_existing_values(grid: &Array2<NonEmptyCell>, guess: &EmptyCell) -> Vec<u8> {
        let mut all_values: Vec<u8> = grid.row(guess.row).map(|x| x.value).to_vec();

        let mut columns_values: Vec<u8> = grid.column(guess.column).iter()
            .map(|x| x.value)
            .filter(|x| !all_values.contains(x)).collect();

        all_values.append(&mut columns_values);

        let mut quadrant_values = grid.iter()
            .filter(|&x| x.quadrant == guess.quadrant)
            .map(|x| x.value)
            .filter(|x| !all_values.contains(x)).collect();

        all_values.append(&mut quadrant_values);

        return all_values.into_iter().filter(|&x| x != 0).collect();
    }

    #[cfg(test)]
    mod tests {
        use crate::utilities::utilities::{get_quadrant_position};

        use super::*;

        #[test]
        fn get_existing_values_test_cell_zero_zero() {
            let grid = generate_grid();

            let mut cell: EmptyCell = EmptyCell::create(0, 0);

            let existing_values = get_existing_values(&grid, &cell);

            assert_eq!(existing_values, [1, 2, 3, 4, 5, 6, 7, 8]);
            eliminate_existing_values(existing_values, &mut cell);
            assert_eq!(vec![9], cell.values);
        }

        #[test]
        fn get_existing_values_test_cell_one_one() {
            let grid = generate_grid();

            let mut cell: EmptyCell = EmptyCell::create(1, 1);

            let existing_values: Vec<u8> = get_existing_values(&grid, &cell);

            assert!(iters_equal_any_order(existing_values.clone().into_iter(), vec![1, 2, 8, 7].into_iter()));
            eliminate_existing_values(existing_values, &mut cell);
            assert!(iters_equal_any_order(cell.values.into_iter(), vec![3, 4, 5, 6, 9].into_iter()));
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
            assert!(guesses.contains(&EmptyCell::new(1, 1, [3, 4, 5, 6, 9].to_vec())));
            assert!(guesses.contains(&EmptyCell::new(1, 2, [3, 4, 5, 6, 9].to_vec())));
            assert!(guesses.contains(&EmptyCell::new(2, 1, [3, 4, 5, 6, 9].to_vec())));
            assert!(guesses.contains(&EmptyCell::new(2, 2, [3, 4, 5, 6, 9].to_vec())));
            assert!(guesses.contains(&EmptyCell::new(8, 8, [2, 3, 4, 5, 6, 7, 9].to_vec())));
        }
    }
}
