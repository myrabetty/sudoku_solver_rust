use crate::model::model::{EmptyCell, Guess};

//```in this module there are all functions that helps th solver to find a solution
pub(crate) mod solver_helper {
    use crate::model::model::{EmptyCell, Guess, NonEmptyCell};
    use crate::utilities::utilities::get_quadrant_position;
    use logs::{debug};
    //Determines the guess which has the smallest set of probabilities as follows:
// it finds the value N-tuple of values that is in a N-tuple of cell and select a value in
// the N-tuple.
// returns the cell with a value that is to be added in the grid.
    pub(crate) fn find_new_guess(allowed_values: &Vec<EmptyCell>) -> Result<Guess, ()> {

        //find first empty cell with one only option
        debug!("trying to find cell with only one option");
        match returns_first_unique_guess(&allowed_values) {
            Some(guessed_value) => return Ok(guessed_value),
            None => {}
        }

        for number_of_appearances in 1..9 {
            debug!("trying to find value which can be in only {:} cells", number_of_appearances);
            match returns_cell_with_unique_value(&allowed_values, number_of_appearances) {
                Some(guessed_value) => return Ok(guessed_value),
                None => {}
            }
        }
    return Err(());
}

//``` given a vector of allowed values, returns the position of the first element that contain
// only one allowed value.
fn returns_first_unique_guess(allowed_values: &Vec<EmptyCell>) -> Option<Guess> {
    let index = allowed_values.iter()
        .position(|guess| (*guess).values.len() == 1);

    return match index {
        Some(index) => {
            let guess = Guess {
                row: allowed_values[index].row,
                column: allowed_values[index].column,
                value: allowed_values[index].values[0],
            };
            Some(guess)
        }
        None => None
    };
}

//find if there is a value that appears in one empty cell only in a row
fn returns_cell_with_unique_value(allowed_values: &Vec<EmptyCell>, wanted_number_of_appearance: usize) -> Option<Guess> {
    for value in 1..10_u8 {
        for index in 0..9_usize {
            match number_of_appearances(&allowed_values, &value, index, wanted_number_of_appearance, (|x: &EmptyCell, value: &u8, index: usize| (*x).row == index && (*x).values.contains(value))) {
                Some(guessed_value) => return Some(guessed_value),
                None => {}
            }
            match number_of_appearances(&allowed_values, &value, index, wanted_number_of_appearance,(|x: &EmptyCell, value: &u8, index: usize| (*x).column == index && (*x).values.contains(value))) {
                Some(guessed_value) => return Some(guessed_value),
                None => {}
            }
            match number_of_appearances(&allowed_values, &value, index, wanted_number_of_appearance,(|x: &EmptyCell, value: &u8, index: usize| (*x).quadrant == index && (*x).values.contains(value))) {
                Some(guessed_value) => return Some(guessed_value),
                None => {}
            }
        }
    }
    return None;
}

    fn number_of_appearances(guesses: &Vec<EmptyCell>, value: &u8, index: usize, wanted_number_of_appearance: usize, filter: fn(&EmptyCell, &u8, usize) -> bool) -> Option<Guess> {
        let number_of_appearances = guesses.iter()
            .filter(|&x| filter(&x, &value, index))
            .count();
        if number_of_appearances == wanted_number_of_appearance {
            let result = guesses.iter()
                .filter(|&x| x.values.contains(value))
                .map(
                    |x| Guess {
                        row: x.row,
                        column: x.column,
                        value: *value,
                    }
                ).nth(1);

            return result;
        }
        return None;
    }
}
