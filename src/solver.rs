pub mod solver {
    use ndarray::Array2;

    use crate::initialize_empty_cells::initialize_empty_cells::set_allowed_values;
    use crate::model::model::{EmptyCell, NonEmptyCell};
    use crate::model::model::GridFunctions;
    use crate::solver_helper::solver_helper::select_best_guess;

    pub fn solve(initial_grid: &Array2<NonEmptyCell>) -> Array2<NonEmptyCell> {
        let mut current_grid: Array2<NonEmptyCell> = initial_grid.clone();
        let mut complement: Vec<(Array2<NonEmptyCell>, Vec<EmptyCell>)> = Vec::new();


        while !current_grid.is_complete() {
            let guesses = set_allowed_values(&current_grid);
            if !is_a_guess_empty(&guesses) {
                // (current_grid, guesses) = complement.pop().unwrap();
                //  current_grid = com.grid;
            }

            // whatever happens before I arrive here with a set of allowed values and a grid.
            // value = find_new_guess(current_grid, guesses);
        }

        return current_grid;
    }

    fn is_a_guess_empty(guesses: &Vec<EmptyCell>) -> bool {
        return guesses.iter().any(|x| x.values.is_empty());
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
