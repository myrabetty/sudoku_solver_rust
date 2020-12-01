use crate::model::model::{EmptyCell, Guess};

//```in this module there are all functions that helps th solver to find a solution
pub(crate) mod solver_helper {
    use logs::debug;
    use ndarray::Array2;

    use crate::model::model::{EmptyCell, EmptyCellFunctions, Guess, NonEmptyCell};
    use crate::utilities::utilities::get_quadrant_position;

    //Determines the guess which has the smallest set of probabilities as follows:
// it finds the value N-tuple of values that is in a N-tuple of cell and select a value in
// the N-tuple.
// returns the cell with a value that is to be added in the grid.
    pub(crate) fn find_new_guess(allowed_values: &Vec<EmptyCell>) -> Result<Guess, ()> {
        debug!("eliminate guesses for values that appear in only one row/column within the quadrant");

        match apply_one_cell_strategies(&allowed_values) {
            Some(guessed_value) => return Ok(guessed_value),
            None => {}
        }
        match apply_two_cells_strategy(&allowed_values) {
            Some(guessed_value) => return Ok(guessed_value),
            None => {}
        }


        return Err(());
    }


    pub fn apply_one_cell_strategies(allowed_values: &Vec<EmptyCell>) -> Option<Guess> {
        //find first empty cell with one only option
        debug!("trying to find cell with only one option");
        match returns_first_unique_guess(&allowed_values) {
            Some(guessed_value) => return Some(guessed_value),
            None => {}
        }

        debug!("trying to find value which can be in only one cells");
        match returns_cell_with_unique_value(&allowed_values) {
            Some(guessed_value) => return Some(guessed_value),
            None => {}
        }
        return None;
    }

    pub fn initialize_empty_values(grid: &Array2<NonEmptyCell>) -> Vec<EmptyCell> {
        let mut guesses: Vec<EmptyCell> = Vec::new();
        for i in 0..9 {
            for j in 0..9 {
                if grid[[i, j]].value == 0 {
                    let guess = EmptyCell::with_all_values(i, j);
                    guesses.push(guess);
                }
            }
        }
        return guesses;
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

    //find if there is a value that appears in one empty cell only in a row
    fn returns_cell_with_unique_value(allowed_values: &Vec<EmptyCell>) -> Option<Guess> {
        for value in 1..10_u8 {
            for index in 0..9_usize {
                // debug!("checking number of appearances for value {:} in row {:}", value, index);
                match find_unique_appearances(&allowed_values, &value, index, (|x: &EmptyCell, value: &u8, index: usize| (*x).row == index && (*x).values.contains(value))) {
                    Some(guessed_value) => return Some(guessed_value),
                    None => {}
                }

                // debug!("checking number of appearances for value {:} in column {:}", value, index);
                match find_unique_appearances(&allowed_values, &value, index, (|x: &EmptyCell, value: &u8, index: usize| (*x).column == index && (*x).values.contains(value))) {
                    Some(guessed_value) => return Some(guessed_value),
                    None => {}
                }
                // debug!("checking number of appearances for value {:} in quadrant {:}", value, index);
                match find_unique_appearances(&allowed_values, &value, index, (|x: &EmptyCell, value: &u8, index: usize| (*x).quadrant == index && (*x).values.contains(value))) {
                    Some(guessed_value) => return Some(guessed_value),
                    None => {}
                }
            }
        }
        return None;
    }

    fn find_unique_appearances(guesses: &Vec<EmptyCell>, value: &u8, index: usize, filter: fn(&EmptyCell, &u8, usize) -> bool) -> Option<Guess> {
        let number_of_appearances = guesses.iter()
            .filter(|&x| filter(&x, &value, index))
            .count();
        // debug!("number of appearances for value {:} is {:}", value, number_of_appearances);
        if number_of_appearances == 1 {
            let result = guesses.iter()
                .filter(|&x| filter(&x, &value, index))
                .map(
                    |x| Guess {
                        row: x.row,
                        column: x.column,
                        value: *value,
                    }
                ).nth(0);

            return result;
        }
        return None;
    }


    // will remove the values if any that appears in only one row/column within a quadrant from the other quadrants
    // and same row/column and then
    fn apply_two_cells_strategy(allowed_values: &Vec<EmptyCell>) -> Option<Guess> {
        // finds a value that is in the same qudrant appearing twice in a column or row only
        // if it finds such a value it removed the value from the remaining segment of row or column on the other quadrants.
        for value in 1..10{
            for quadrant in 0..9 {
                for row in 0..9{}

                for column in 0..9 {}
            }
    }
    return None;
}

#[cfg(test)]
mod tests {}
}
