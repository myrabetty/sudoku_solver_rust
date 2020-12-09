use itertools::Itertools;
use logs::debug;

use crate::core::model::EmptyCell;

pub fn x_wing_strategy(mut allowed_values: &mut &mut Vec<EmptyCell>) {
    for value in 1..10_u8 {
        for index_1 in 0..8_usize {
            for index_2 in 0..8_usize {
                match find_x_wing_pairs_rows(&mut allowed_values, value, index_1, index_2) {
                    true => {
                        debug!("found x wing for value {:} and rows {:}, {:}", value, index_1, index_2);
                    }
                    _ => {}
                }

                match find_x_wing_pairs_columns(&mut allowed_values, value, index_1, index_2) {
                    true => { debug!("found x wing for value {:} and columns {:}, {:}", value, index_1, index_2); }
                    _ => {}
                }
            }
        }
    }
}

fn find_x_wing_pairs_rows(allowed_values: &mut Vec<EmptyCell>, value: u8, row_1: usize, row_2: usize) -> bool {
    let values: Vec<EmptyCell> = allowed_values.clone()
        .into_iter()
        .filter(|x| (x.row == row_1 || x.row == row_2) && x.values.contains(&value)).collect();

    //find unique column appearances if they are 2 it means they appear in the same columns
    let unique_indexes: Vec<usize> = values.clone().iter().map(|x| x.column).unique().collect();
    if unique_indexes.len() == 2 {
        allowed_values.iter_mut().for_each(|x|
            if x.column == unique_indexes[0] || x.column == unique_indexes[1] {
                let new_values = x.values
                    .clone()
                    .into_iter()
                    .filter(|&x| x != value).collect();
                x.values = new_values;
            }
        );
        return true;
    }
    return false;
}

fn find_x_wing_pairs_columns(allowed_values: &mut Vec<EmptyCell>, value: u8, column_1: usize, column_2: usize) -> bool {
    let values: Vec<EmptyCell> = allowed_values.clone()
        .into_iter()
        .filter(|x| (x.column == column_1 || x.column == column_2) && x.values.contains(&value)).collect();

    //find unique column appearances if they are 2 it means they appear in the same columns
    let unique_indexes: Vec<usize> = values.clone().iter().map(|x| x.row).unique().collect();
    if unique_indexes.len() == 2 {
        allowed_values.iter_mut().for_each(|x|
            if x.row == unique_indexes[0] || x.row == unique_indexes[1] {
                let new_values = x.values
                    .clone()
                    .into_iter()
                    .filter(|&x| x != value).collect();
                x.values = new_values;
            }
        );
        return true;
    }
    return false;
}
