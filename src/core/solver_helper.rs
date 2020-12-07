use itertools::{all, Itertools};
use logs::debug;
use ndarray::Array2;

use crate::core::model::{EmptyCell, EmptyCellFunctions, Guess, NonEmptyCell};

//```in this module there are all functions that helps th solver to find a solution

//Determines the guess which has the smallest set of probabilities as follows:
// it finds the value N-tuple of values that is in a N-tuple of cell and select a value in
// the N-tuple.
// returns the cell with a value that is to be added in the grid.
pub(crate) fn find_new_guess(mut allowed_values: &mut Vec<EmptyCell>) -> Result<Guess, ()> {
    debug!("eliminate guesses for values that appear in only one row/column within the quadrant");

    match apply_one_cell_strategies(&allowed_values) {
        Some(guessed_value) => return Ok(guessed_value),
        None => {}
    }

    debug!("allowed values before applying two points strategy {:?}", allowed_values);

    match apply_two_cells_strategy(&mut allowed_values) {
        Some(guessed_value) => return Ok(guessed_value),
        None => {}
    }

    match try_jumping(&mut allowed_values) {
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
            match find_unique_appearances(&allowed_values, &value, index, (|x: &EmptyCell, index: usize| (*x).row == index)) {
                Some(guessed_value) => return Some(guessed_value),
                None => {}
            }

            // debug!("checking number of appearances for value {:} in column {:}", value, index);
            match find_unique_appearances(&allowed_values, &value, index, (|x: &EmptyCell, index: usize| (*x).column == index)) {
                Some(guessed_value) => return Some(guessed_value),
                None => {}
            }
            // debug!("checking number of appearances for value {:} in quadrant {:}", value, index);
            match find_unique_appearances(&allowed_values, &value, index, (|x: &EmptyCell, index: usize| (*x).quadrant == index)) {
                Some(guessed_value) => return Some(guessed_value),
                None => {}
            }
        }
    }
    return None;
}

fn find_unique_appearances(guesses: &Vec<EmptyCell>, value: &u8, index: usize, filter: fn(&EmptyCell, usize) -> bool) -> Option<Guess> {
    let number_of_appearances = guesses.iter()
        .filter(|&x| filter(&x, index) && (*x).values.contains(value))
        .count();
    // debug!("number of appearances for value {:} is {:}", value, number_of_appearances);
    if number_of_appearances == 1 {
        let result = guesses.iter()
            .filter(|&x| filter(&x, index) && (*x).values.contains(value))
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
fn apply_two_cells_strategy(mut allowed_values: &mut Vec<EmptyCell>) -> Option<Guess> {
    // finds a value that is in the same qudrant appearing twice in a column or row only
    // if it finds such a value it removed the value from the remaining segment of row or column on the other quadrants.
    // probably we do not need to clone this we can just update the array.
    for value in 1..10_u8 {
        for quadrant in 0..9 {
            let quadrant_values: Vec<EmptyCell> = allowed_values.clone()
                .into_iter()
                .filter(|x| x.quadrant == quadrant && x.values.contains(&value))
                .collect();

            match return_doublet_in_one_location(&quadrant_values, |x: &EmptyCell| x.row) {
                Some(row) => {
                    debug!("Analyzing quadrant: {:}. Value {:} that can appear only in this row {:}", quadrant, value, row);
                    remove_value_from_other_segments(&mut allowed_values, row, quadrant, value, |x: &EmptyCell, row, quadrant| x.row == row && x.quadrant != quadrant);
                    match apply_one_cell_strategies(&allowed_values) {
                        Some(guess) => return Some(guess),
                        None => {}
                    }
                }
                None => {}
            }

            match return_doublet_in_one_location(&quadrant_values, |x: &EmptyCell| x.column) {
                Some(column) => {
                    debug!("Analyzing quadrant: {:}. Value {:} that can appear only in this column {:}", quadrant, value, column);
                    remove_value_from_other_segments(&mut allowed_values, column, quadrant, value, |x: &EmptyCell, column, quadrant| x.column == column && x.quadrant != quadrant);
                    match apply_one_cell_strategies(&allowed_values) {
                        Some(guess) => return Some(guess),
                        None => {}
                    }
                }
                None => {}
            }
        }
    }

    for value_1 in 1..10_u8 {
        for value_2 in (value_1 + 1)..10_u8 {
            for index in 0..8_usize {
                match find_two_values_in_only_two_places(&mut allowed_values, value_1, value_2, index, |x: &EmptyCell, index| x.row == index) {
                    Some(index) => {
                        debug!("Found unique doublet {:} {:} in row {:}", value_1, value_2, index);
                        match apply_one_cell_strategies(&allowed_values) {
                            Some(guess) => return Some(guess),
                            None => {}
                        }
                    }
                    None => {}
                }
                match find_two_values_in_only_two_places(&mut allowed_values, value_1, value_2, index, |x: &EmptyCell, index| x.column == index) {
                    Some(index) => {
                        debug!("Found unique doublet {:} {:} in column {:}", value_1, value_2, index);
                        match apply_one_cell_strategies(&allowed_values) {
                            Some(guess) => return Some(guess),
                            None => {}
                        }
                    }
                    None => {}
                }
            }
        }
    }
    debug!("this is the array of allowed values {:?}", allowed_values);
    return None;
}

fn remove_value_from_other_segments(allowed_values: &mut Vec<EmptyCell>, location: usize, quadrant: usize, value: u8, filter: fn(&EmptyCell, usize, usize) -> bool) {
    allowed_values
        .iter_mut()
        .for_each(|x| {
            if filter(x, location, quadrant) {
                x.eliminate(value)
            }
        });
}

fn return_doublet_in_one_location(quadrant_values: &Vec<EmptyCell>, transform: fn(&EmptyCell) -> usize) -> Option<usize> {
    let locations: Vec<usize> = quadrant_values.iter().map(|x| transform(x)).unique().collect();

    match locations.len() == 1 && quadrant_values.len() > 1 {
        true => return Some(locations[0]),
        _ => None
    }
}

// finds if there is a cuouple of values that appear in only two cells in a row/column/quadrant if so remove all other values from those cells.
fn find_two_values_in_only_two_places(allowed_values: &mut Vec<EmptyCell>, value_1: u8, value_2: u8, index: usize, filter: fn(&EmptyCell, usize) -> bool) -> Option<usize> {
    let cells_with_value_1 = allowed_values
        .iter()
        .filter(|x| filter(x, index) && x.values.contains(&value_1)).count();

    let cells_with_value_2 = allowed_values
        .iter()
        .filter(|x| filter(x, index) && x.values.contains(&value_2)).count();

    let cells_with_couple: Vec<usize> = allowed_values
        .iter()
        .positions(|x| filter(x, index) && x.values.contains(&value_1) && x.values.contains(&value_2)).collect();
    if cells_with_couple.len() == 2 && cells_with_value_1 == 2 && cells_with_value_2 == 2 {
        allowed_values[cells_with_couple[0]].values = vec![value_1, value_2];
        allowed_values[cells_with_couple[1]].values = vec![value_1, value_2];
        return Some(index);
    }
    return None;
}

fn try_jumping(allowed_values: &Vec<EmptyCell>) -> Option<Guess> {
    for element in allowed_values {
        if element.values.len() == 2 {
            let guess = Guess { row: element.row, column: element.column, value: element.values[0] };
            return Some(guess);
        }
    }

    return None;
}

#[cfg(test)]
mod tests {}
