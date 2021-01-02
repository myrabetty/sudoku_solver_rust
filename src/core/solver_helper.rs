use itertools::{Itertools};
use log::debug;
use ndarray::Array2;

use crate::core::hidden_tuplets_strategies::{hidden_pairs_strategy, hidden_triplets_strategy};
use crate::core::model::{EmptyCell, EmptyCellFunctions, Guess, NonEmptyCell};
use crate::core::naked_tuplets_stategies::naked_pairs_strategy;
use crate::core::x_wing_strategy::x_wing_strategy;
use crate::core::utilities::iters_equal_any_order;

//```in this module there are all functions that helps th solver to find a solution

//Determines the guess which has the smallest set of probabilities as follows:
// it finds the value N-tuple of values that is in a N-tuple of cell and select a value in
// the N-tuple.
// returns the cell with a value that is to be added in the grid.
// it applies only deterministic algorithms
pub(crate) fn find_new_guess(mut allowed_values: &mut Vec<EmptyCell>) -> Result<Guess, ()> {
    debug!("eliminate guesses for values that appear in only one row/column within the quadrant");

    match apply_one_cell_strategies(&allowed_values) {
        Some(guessed_value) => return Ok(guessed_value),
        None => {}
    }
    match apply_two_cells_strategy(&mut allowed_values) {
        Some(guessed_value) => return Ok(guessed_value),
        None => {}
    }
    return Err(());
}


fn apply_one_cell_strategies(allowed_values: &Vec<EmptyCell>) -> Option<Guess> {
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
            match find_unique_appearences(&allowed_values, &value, index, |x: &EmptyCell, index: usize| (*x).row == index) {
                Some(guessed_value) => return Some(guessed_value),
                None => {}
            }

            // debug!("checking number of appearances for value {:} in column {:}", value, index);
            match find_unique_appearences(&allowed_values, &value, index, |x: &EmptyCell, index: usize| (*x).column == index) {
                Some(guessed_value) => return Some(guessed_value),
                None => {}
            }
            // debug!("checking number of appearances for value {:} in quadrant {:}", value, index);
            match find_unique_appearences(&allowed_values, &value, index, |x: &EmptyCell, index: usize| (*x).quadrant == index) {
                Some(guessed_value) => return Some(guessed_value),
                None => {}
            }
        }
    }
    return None;
}

fn find_unique_appearences(guesses: &Vec<EmptyCell>, value: &u8, index: usize, filter: fn(&EmptyCell, usize) -> bool) -> Option<Guess> {
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
    // we need to apply this set of astrategies over and over again. Only stop if they removec nothing.

    let mut guess: Option<Guess> = None;
    let mut values_are_updated = true;
    while guess.is_none() && values_are_updated {
        let allowed_values_old = allowed_values.clone();
        remove_values_in_one_location_only(&mut allowed_values);
        hidden_pairs_strategy(&mut allowed_values);
        naked_pairs_strategy(&mut allowed_values);
        hidden_triplets_strategy(&mut allowed_values);
        x_wing_strategy(&mut allowed_values);

        values_are_updated = false;
        for i in 0..allowed_values_old.len() {
            if allowed_values_old[i].values != allowed_values[i].values {
                values_are_updated = true;
                break;
            }
        }

        guess = apply_one_cell_strategies(&allowed_values);
    }
    return guess;
}

// finds a value that is in the same quadrant appearing at least twice in a column or row
// if it finds such a value it removes the value from the remaining segment of row or column in the other quadrants
fn remove_values_in_one_location_only(mut allowed_values: &mut &mut Vec<EmptyCell>) {
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
                }
                None => {}
            }

            match return_doublet_in_one_location(&quadrant_values, |x: &EmptyCell| x.column) {
                Some(column) => {
                    debug!("Analyzing quadrant: {:}. Value {:} that can appear only in this column {:}", quadrant, value, column);
                    remove_value_from_other_segments(&mut allowed_values, column, quadrant, value, |x: &EmptyCell, column, quadrant| x.column == column && x.quadrant != quadrant);
                }
                None => {}
            }
        }
    }
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

//```given a set of empty values  if these quadrant_values are all in the same row/column returns that row/column
fn return_doublet_in_one_location(quadrant_values: &Vec<EmptyCell>, transform: fn(&EmptyCell) -> usize) -> Option<usize> {
    // returns the distinct row/ or columns the value appears.
    let locations: Vec<usize> = quadrant_values.iter().map(|x| transform(x)).unique().collect();

    // if it appears only in one distinct location (row or column) in the quadrant and multiple times in the quadrant
    // then it is appearing at least twice in only only one row/column so we return the row/column where it appears.
    match locations.len() == 1 && quadrant_values.len() > 1 {
        true => return Some(locations[0]),
        _ => None
    }
}

#[cfg(test)]
mod tests {}
