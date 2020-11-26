//```in this module there are all functions that helps th solver to find a solution
pub(crate) mod solver_helper {
    use crate::model::model::{EmptyCell, Guess, NonEmptyCell};
    use crate::utilities::utilities::get_quadrant_position;

    //Determines the guess which has the smallest set of probabilities as follows:
// it finds the value N-tuple of values that is in a N-tuple of cell and select a value in
// the N-tuple.
// returns the cell with a value that is to be added in the grid.
    pub(crate) fn find_new_guess(allowed_values: &Vec<EmptyCell>) -> Result<Guess, ()> {

        //find first empty cell with one only option
        match returns_first_unique_guess(&allowed_values) {
            Some(guessed_value) => return Ok(guessed_value),
            None => {}
        }

        //find if there is a value that appears in one empty cell only in a row
        for value in 1..10_u8 {
            for index in 0..9_usize {
                match unique_value(&allowed_values, &value, index, (|x: &EmptyCell, value: &u8, index: usize| (*x).row == index && (*x).values.contains(&6))) {
                    Some(guessed_value) => return Ok(guessed_value),
                    None => {}
                }
                /*match unique_value(&allowed_values, value,  index, (|x:EmptyCell, value:u8, index:usize| x.column == index && x.values.contains(*value))) {
                    Some(guessed_value) => return Ok(guessed_value),
                    None => {}
                }
                match unique_value(&allowed_values, value,  index, (|x:EmptyCell, value:u8, index:usize| x.quadrant == index && x.values.contains(*value))) {
                    Some(guessed_value) => return Ok(guessed_value),
                    None => {}
                }*/
            }
        }
        return Err(());
    }

    //``` given a vector of allowed values, returns the position of the first element that contain
    // only one allowed value.
    fn returns_first_unique_guess(allowed_values: &Vec<EmptyCell>) -> Option<Guess> {
        let index = allowed_values.iter()
            .position(|guess| guess.values.len() == 1);

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

    fn unique_value(guesses: &Vec<EmptyCell>, value: &u8, index: usize, filter: fn(&EmptyCell, &u8, usize) -> bool) -> Option<Guess> {
        let number_of_appearances = guesses.iter()
            .filter(|&x| filter(&x, &value, index))
            .count();
        if number_of_appearances == 1 {
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
