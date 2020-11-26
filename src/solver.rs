pub mod solver {
    use std::borrow::Borrow;

    use logs::{debug, info, warn};
    use ndarray::Array2;

    use crate::initialize_empty_cells::initialize_empty_cells::set_allowed_values;
    use crate::model::model::{EmptyCell, Guess, NonEmptyCell};
    use crate::model::model::GridFunctions;
    use crate::solver_helper::solver_helper::find_new_guess;

    pub fn solve(mut grid: Array2<NonEmptyCell>) -> Array2<NonEmptyCell> {
        let mut complements: Vec<(Array2<NonEmptyCell>, Vec<EmptyCell>)> = Vec::new();
        while !grid.is_complete() {
            let mut guesses = set_allowed_values(&grid);
            if is_a_guess_empty(&guesses) {
                warn!("taking complement after ruling solution as invalid");
                let complement = complements.pop().unwrap();
                guesses = complement.1;
                grid = complement.0;
            }

            // whatever happens before I arrive here with a set of allowed values and a grid.
            let result = find_new_guess(&guesses);
            if result.is_ok() {
                let new_value = result.unwrap();
                remove_current_choice(&mut guesses, &new_value);
                complements.push((grid.clone(), guesses));
                info!("value is {:?}", new_value);
                grid[[new_value.row, new_value.column]].value = new_value.value;
            } else {
                panic!("we cannot find a valid guess this should not happen!")
            }
        }

        info!("Grid is complete");
        return grid;
    }

    fn is_a_guess_empty(guesses: &Vec<EmptyCell>) -> bool {
        let result = guesses.iter().position(|x| x.values.is_empty());
        match result {
            Some(index) => {
                warn!("solution is empty set for {:?}", guesses[index]);
                return true;
            }
            None => return false
        }
    }

    // ``` removes the chosen value from the array of possible values
    fn remove_current_choice(empty_cells: &mut Vec<EmptyCell>, choice: &Guess) {
        let index = empty_cells.iter()
            .position(|x| x.row == choice.row && x.column == choice.column).unwrap();

        let values = empty_cells[index]
            .values.clone().into_iter()
            .filter(|&x| x != choice.value).collect();

        empty_cells[index].values = values;
    }

    #[cfg(test)]
    mod tests {
        use crate::model::model::EmptyCellFunctions;

        use super::*;

        #[test]
        fn is_a_guess_empty_test() {
            let mut emptyCell = EmptyCell::create(0, 0);
            let mut guesses = vec![emptyCell];
            assert!(!is_a_guess_empty(&guesses));

            guesses[0].values = vec![];
            assert!(is_a_guess_empty(&guesses));
        }
    }
}

/*
let mut complement_choices_stack: Vec<Choice> = Vec::default();




    //0) compute allowed values
    let mut allowed_values = set_allowed_values(&current_grid).unwrap();

    //1) validate both the grid and the allowed values
    validate_grid(&current_grid);
    match validate_allowed_values(allowed_values) {
        Err(e) => {
            allowed_values = complement_choices_stack.pop().unwrap().allowed_values;
            current_grid = complement_choices_stack.pop().unwrap().grid;
        }
        _ => return
    }

    //2) fill unique possibilities.
    //todo: write this method as an utility and start to move also other used methods in model in that module!

    //3) select the best value for a cell
    let guessed_value = select_best_guess(&allowed_values).unwrap();

    //4) add to the stack the complement choice: i.e. the current grid and removes from the
    // allowed values the current choice.


    //5) add the value to the grid.
    guessed_value.set_guessed_value_in_grid(&mut current_grid);

    grid_stack.push(allowed_values
        .elimination_process(&guessed_value)
        .set_guessed_values_in_grid(&current_grid));

    //3 remove value from allowed_values and adds new allowed values to the stack
    //allowed_values_stack.push(allowed_values.eliminate_guessed_value_from_other_cells(&guessed_value));
}

fn has_not_unique_guesses(guesses: &Vec<AllowedValues>) -> bool {
let result = guesses.iter().map(|&guess| guess.values.len()).find(|&size| size > 1);
return match result {
    Some(_) => true,
    None => false
};
}
}
*/
