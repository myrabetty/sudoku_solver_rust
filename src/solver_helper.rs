//```in this module there are all functions that helps th solver to find a solution
pub(crate) mod solver_helper {
    use crate::model::model::{EmptyCell, Guess, NonEmptyCell};
    use crate::utilities::utilities::get_quadrant_position;

    //Determines the guess which has the smallest set of probabilities as follows:
// it finds the value N-tuple of values that is in a N-tuple of cell and select a value in
// the N-tuple.
// returns the cell with a value that is to be added in the grid.
    pub(crate) fn find_new_guess(allowed_values: &Vec<EmptyCell>) -> Result<Guess, ()> {

        // try to find first an unique value
        match returns_first_unique_guess(&allowed_values) {
            Some(guessed_value) => return Ok(guessed_value),
            None => {}
        }

        //let counter = guesses.returns_hashmap_with_counts();
        //let min = counter.iter().min_by(|&&x, &&y| x.1.len() < y.1.len()).unwrap();

        //```trivial case of a number that is present only in one place.
        /*if min.1.len() == 1 {
        return Ok(*min.1.get(0).unwrap());
    }*/
        // ```case of a number present in two places
        //if
        //return GuessedValue::default();
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
}
