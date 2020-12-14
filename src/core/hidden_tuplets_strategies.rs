use itertools::Itertools;
use logs::debug;

use crate::core::model::EmptyCell;

pub fn hidden_pairs_strategy(mut allowed_values: &mut &mut Vec<EmptyCell>) {
    for value_1 in 1..10_u8 {
        for value_2 in (value_1 + 1)..10_u8 {
            for index in 0..9_usize {
                match find_connected_hidden_pairs(&mut allowed_values, value_1, value_2, index, |x: &EmptyCell, index| x.row == index) {
                    Some(index) => {
                        debug!("Found hidden pair {:} {:} in row {:}", value_1, value_2, index);
                    }
                    None => {}
                }
                match find_connected_hidden_pairs(&mut allowed_values, value_1, value_2, index, |x: &EmptyCell, index| x.column == index) {
                    Some(index) => {
                        debug!("Found hidden pair {:} {:} in column {:}", value_1, value_2, index);
                    }
                    None => {}
                }

                match find_connected_hidden_pairs(&mut allowed_values, value_1, value_2, index, |x: &EmptyCell, index| x.quadrant == index) {
                    Some(index) => {
                        debug!("Found hidden pair {:} {:} in quadrant {:}", value_1, value_2, index);
                    }
                    None => {}
                }
            }
        }
    }
}

pub fn hidden_triplets_strategy(mut allowed_values: &mut &mut Vec<EmptyCell>) {
    for value_1 in 1..10_u8 {
        for value_2 in (value_1 + 1)..10_u8 {
            for value_3 in (value_2 + 1)..10_u8 {
                for index in 0..9_usize {
                    match find_connected_hidden_triplets(&mut allowed_values, value_1, value_2, value_3, index, |x: &EmptyCell, index| x.row == index) {
                        Some(index) => {
                            debug!("Found hidden triplet {:} {:} {:} in row {:}", value_1, value_2, value_3, index);
                        }
                        None => {}
                    }
                    match find_connected_hidden_triplets(&mut allowed_values, value_1, value_2, value_3, index, |x: &EmptyCell, index| x.column == index) {
                        Some(index) => {
                            debug!("Found hidden triplet {:} {:} {:} in column {:}", value_1, value_2, value_3, index);
                        }
                        None => {}
                    }

                    match find_connected_hidden_triplets(&mut allowed_values, value_1, value_2, value_3, index, |x: &EmptyCell, index| x.quadrant == index) {
                        Some(index) => {
                            debug!("Found hidden triplet {:} {:} {:} in quadrant {:}", value_1, value_2, value_3, index);
                        }
                        None => {}
                    }
                }
            }
        }
    }
}

// finds if there is a triplet of values that appear in only three cells in a row/column/quadrant if so remove all other values from those cells.
fn find_connected_hidden_triplets(allowed_values: &mut Vec<EmptyCell>, value_1: u8, value_2: u8, value_3: u8, index: usize, filter: fn(&EmptyCell, usize) -> bool) -> Option<usize> {
    let cells_with_value_1 = allowed_values
        .iter()
        .filter(|x| filter(x, index) && x.values.contains(&value_1)).count();

    let cells_with_value_2 = allowed_values
        .iter()
        .filter(|x| filter(x, index) && x.values.contains(&value_2)).count();

    let cells_with_value_3 = allowed_values
        .iter()
        .filter(|x| filter(x, index) && x.values.contains(&value_3)).count();

    let cells_with_triplets: Vec<usize> = allowed_values
        .iter()
        .positions(|x| filter(x, index) && x.values.contains(&value_1) && x.values.contains(&value_2) && x.values.contains(&value_3)).collect();
    if cells_with_triplets.len() == 3 && cells_with_value_1 == 3 && cells_with_value_2 == 3 && cells_with_value_3 == 3 {
        allowed_values[cells_with_triplets[0]].values = vec![value_1, value_2, value_3];
        allowed_values[cells_with_triplets[1]].values = vec![value_1, value_2, value_3];
        allowed_values[cells_with_triplets[2]].values = vec![value_1, value_2, value_3];
        return Some(index);
    }
    return None;
}

// finds if there is a pair of values that appear in only two cells in a row/column/quadrant if so remove all other values from those cells.
fn find_connected_hidden_pairs(allowed_values: &mut Vec<EmptyCell>, value_1: u8, value_2: u8, index: usize, filter: fn(&EmptyCell, usize) -> bool) -> Option<usize> {
    let cells_with_value_1 = allowed_values
        .iter()
        .filter(|x| filter(x, index) && x.values.contains(&value_1)).count();

    let cells_with_value_2 = allowed_values
        .iter()
        .filter(|x| filter(x, index) && x.values.contains(&value_2)).count();


    let cells_with_pairs: Vec<usize> = allowed_values
        .iter()
        .positions(|x| filter(x, index) && x.values.contains(&value_1) && x.values.contains(&value_2)).collect();
    if cells_with_pairs.len() == 2 && cells_with_value_1 == 2 && cells_with_value_2 == 2 {
        allowed_values[cells_with_pairs[0]].values = vec![value_1, value_2];
        allowed_values[cells_with_pairs[1]].values = vec![value_1, value_2];
        return Some(index);
    }
    return None;
}
