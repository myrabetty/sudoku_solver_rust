//```in this module there are all functions that helps th solver to find a solution
pub(crate) mod solver_helper {
    use crate::model::model::{EmptyCell, GuessCell};

    //Determines the guess which has the smallest set of probabilities as follows:
// it finds the value N-tuple of values that is in a N-tuple of cell and select a value in
// the N-tuple.
    pub(crate) fn select_best_guess(allowed_values: &Vec<EmptyCell>) -> Result<GuessCell, ()> {

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

    //``` given a vector of allowed values, returns the first element that contain
    // only one allowed value.
    fn returns_first_unique_guess(guesses: &Vec<EmptyCell>) -> Option<GuessCell> {
        return guesses.iter()
            .filter(|&guess| guess.values.len() == 1)
            .map(|guess| return GuessCell {
                row: guess.row,
                column: guess.column,
                value: *guess.values.get(0).unwrap(),
            })
            .nth(1);
    }
}
