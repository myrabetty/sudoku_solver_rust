use ndarray::Array2;
use crate::core::model::{NonEmptyCell, EmptyCell, GridFunctions};

// ```given a grid returns for each empty cell the possible values.
pub fn remove_placed_values(grid: &Array2<NonEmptyCell>, guesses: &mut Vec<EmptyCell>) {
    guesses.iter_mut().for_each(|mut guess| {
        let existing_values = get_existing_values(grid, guess);
        eliminate_existing_values(existing_values, &mut guess);
    });
}

fn eliminate_existing_values(existing_values: Vec<u8>, guess: &mut EmptyCell) {
    guess.values.retain(|x| !existing_values.contains(x));
}

fn get_existing_values(grid: &Array2<NonEmptyCell>, guess: &EmptyCell) -> Vec<u8> {
    let mut existing_values: Vec<u8> = vec![];
    for i in 0..9 {
        for j in 0..9 {
            if (i == guess.row || j == guess.column || grid[[i, j]].quadrant == guess.quadrant) && grid[[i, j]].value != 0 {
                existing_values.push(grid[[i, j]].value);
            }
        }
    }
    return existing_values;
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::core::model::EmptyCellFunctions;
    use crate::core::utilities::{iters_equal_any_order, get_quadrant};
    use crate::core::solver_helper::initialize_empty_values;

    #[test]
    fn get_existing_values_test_cell_zero_zero() {
        let grid = generate_grid();

        let mut cell: EmptyCell = EmptyCell::with_all_values(0, 0);

        let existing_values = get_existing_values(&grid, &cell);

        eliminate_existing_values(existing_values, &mut cell);
        assert_eq!(vec![9], cell.values);
    }

    #[test]
    fn get_existing_values_test_cell_one_one() {
        let grid = generate_grid();

        let mut cell: EmptyCell = EmptyCell::with_all_values(1, 1);

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
        let mut guesses = initialize_empty_values(&grid);
        remove_placed_values(&grid, &mut guesses);

        assert_eq!(guesses.len(), 65);
        assert!(guesses.contains(&EmptyCell::new(0, 0, [9].to_vec())));
        assert!(guesses.contains(&EmptyCell::new(1, 1, [3, 4, 5, 6, 9].to_vec())));
        assert!(guesses.contains(&EmptyCell::new(1, 2, [3, 4, 5, 6, 9].to_vec())));
        assert!(guesses.contains(&EmptyCell::new(2, 1, [3, 4, 5, 6, 9].to_vec())));
        assert!(guesses.contains(&EmptyCell::new(2, 2, [3, 4, 5, 6, 9].to_vec())));
        assert!(guesses.contains(&EmptyCell::new(8, 8, [2, 3, 4, 5, 6, 7, 9].to_vec())));
    }
}

