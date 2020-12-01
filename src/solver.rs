pub mod solver {
    use std::borrow::Borrow;

    use logs::{debug, info, warn};
    use ndarray::Array2;

    use crate::initialize_empty_cells::initialize_empty_cells::remove_placed_values;
    use crate::model::model::{EmptyCell, Guess, NonEmptyCell};
    use crate::model::model::GridFunctions;
    use crate::solver_helper::solver_helper::{find_new_guess, initialize_empty_values};
    use crate::utilities::utilities::get_quadrant_position;

    pub fn solve(mut grid: Array2<NonEmptyCell>) -> Array2<NonEmptyCell> {
        let mut complements: Vec<(Array2<NonEmptyCell>, Vec<EmptyCell>, Guess)> = Vec::new();
        let mut guesses: Vec<EmptyCell> = initialize_empty_values(&grid);
        remove_placed_values(&grid, &mut guesses);
        while !grid.is_complete() {
            if  guesses.is_empty() || is_a_guess_empty(&guesses) {
                let complement = complements.pop().unwrap();
                warn!("taking complement for choice {:?} after ruling solution as invalid", complement.2);
                guesses = complement.1;
                grid = complement.0;
                //debug!("grid at this check point is {:?}", grid);
            }

            // whatever happens before I arrive here with a set of allowed values and a grid.
            let result = find_new_guess(&guesses);
            if result.is_ok() {
                let new_value = result.unwrap();
                info!("value is {:?}", new_value);
                let complement_guesses = get_complement_choice(&guesses, &new_value);

                if !complement_guesses.is_empty() {
                    complements.push((grid.clone(), complement_guesses, new_value.clone()));
                }

                prepare_to_next_iteration(&mut guesses, &mut grid, &new_value);
            } else {
                panic!("we cannot find a valid guess this should not happen!")
            }
        }

        info!("Grid is complete");
        return grid;
    }

    fn prepare_to_next_iteration(empty_cells: &mut Vec<EmptyCell>, grid: &mut Array2<NonEmptyCell>, new_value: &Guess) {

        // removes the assigned value form the array of empty cells.
        let index = empty_cells.iter()
            .position(|x| x.row == new_value.row && x.column == new_value.column).unwrap();
        empty_cells.remove(index);

        // set the value in the grid
        grid[[new_value.row, new_value.column]].value = new_value.value;

        // removes the value from array of possible values of the empty cells in same row, column or quadrant of the new_value
        empty_cells
            .iter_mut()
            .for_each(|mut x| {
                if x.row == new_value.row || x.column == new_value.column || get_quadrant_position(new_value.row, new_value.column) == x.quadrant {
                    x.eliminate(new_value.value)
                }
            });

    }

    fn is_a_guess_empty(guesses: &Vec<EmptyCell>) -> bool {
        let result = guesses.iter().position(|x| x.values.is_empty());
        return match result {
            Some(index) => {
                warn!("solution is empty set for {:?}", guesses[index]);
                true
            }
            None => false
        };
    }

    // ``` removes the chosen value from the array of possible values
    fn get_complement_choice(current_options: &Vec<EmptyCell>, choice: &Guess) -> Vec<EmptyCell> {
        let mut empty_cells = current_options.clone();
        let index = empty_cells.iter()
            .position(|x| x.row == choice.row && x.column == choice.column).unwrap();

        // debug!("value to remove is {:?}", choice);
        let values: Vec<u8> = empty_cells[index]
            .values.clone().into_iter()
            .filter(|&x| x != choice.value).collect();
        // debug!("before removing the value we have {:?}", empty_cells[index]);
        if values.len() == 0 {
            empty_cells.remove(index);
        } else {
            empty_cells[index].values = values;
            // debug!("after removing the value we have {:?}", empty_cells[index]);
        };
        return empty_cells;
    }

    #[cfg(test)]
    mod tests {
        use crate::model::model::EmptyCellFunctions;
        use crate::utilities::utilities::iters_equal_any_order;

        use super::*;

        #[test]
        fn is_a_guess_empty_test() {
            let emptyCell = EmptyCell::with_all_values(0, 0);
            let mut guesses = vec![emptyCell];
            assert!(!is_a_guess_empty(&guesses));

            guesses[0].values = vec![];
            assert!(is_a_guess_empty(&guesses));
        }

        #[test]
        fn remove_current_choice_test() {
            let mut emptyCell = EmptyCell::with_all_values(0, 0);
            let mut guesses = vec![emptyCell];
            let mut new_guess: Guess = Guess { row: 0, column: 0, value: 9 };
            assert!(iters_equal_any_order(guesses[0].values.clone().into_iter(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9].into_iter()));

            get_complement_choice(&mut guesses, &new_guess);
            assert!(iters_equal_any_order(guesses[0].values.clone().into_iter(), vec![1, 2, 3, 4, 5, 6, 7, 8].into_iter()));
        }
    }
}
