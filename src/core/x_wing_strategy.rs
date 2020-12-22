use itertools::Itertools;
use log::debug;

use crate::core::model::EmptyCell;

pub fn x_wing_strategy(mut allowed_values: &mut &mut Vec<EmptyCell>) {
    for value in 1..10_u8 {
        for index_1 in 0..9_usize {
            for index_2 in index_1 + 1..9_usize {
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
    let values_row_1: Vec<EmptyCell> = allowed_values.clone()
        .into_iter()
        .filter(|x| (x.row == row_1) && x.values.contains(&value)).collect();

    let values_row_2: Vec<EmptyCell> = allowed_values.clone()
        .into_iter()
        .filter(|x| (x.row == row_2) && x.values.contains(&value)).collect();

    //find unique column appearances if they are 2 it means they appear in the same columns
    let unique_indexes_row_1: Vec<usize> = values_row_1.clone().iter().map(|x| x.column).unique().collect();
    let unique_indexes_row_2: Vec<usize> = values_row_2.clone().iter().map(|x| x.column).unique().collect();
    if unique_indexes_row_1 == unique_indexes_row_2 && unique_indexes_row_1.len() == 2 {
        allowed_values.iter_mut().for_each(|x|
            // if the cell is in one of the same columns but different rows the we eliminate the value
            if (x.column == unique_indexes_row_1[0] || x.column == unique_indexes_row_1[1]) && x.row != row_1 && x.row != row_2 {
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
    let values_column_1: Vec<EmptyCell> = allowed_values.clone()
        .into_iter()
        .filter(|x| (x.column == column_1) && x.values.contains(&value)).collect();

    let values_column_2: Vec<EmptyCell> = allowed_values.clone()
        .into_iter()
        .filter(|x| (x.column == column_2) && x.values.contains(&value)).collect();

    //find if the value appears in two unique rows.
    let unique_indexes_column_1: Vec<usize> = values_column_1.clone().iter().map(|x| x.row).unique().collect();
    let unique_indexes_column_2: Vec<usize> = values_column_2.clone().iter().map(|x| x.row).unique().collect();
    if unique_indexes_column_1 == unique_indexes_column_2 && unique_indexes_column_1.len() == 2 {
        allowed_values.iter_mut().for_each(|x|
            // if the cell is in the same rows but different columns the we eliminate the value
            if (x.row == unique_indexes_column_1[0] || x.row == unique_indexes_column_1[1]) && x.column != column_1 && x.column != column_2 {
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


#[cfg(test)]
mod tests {
    use itertools::{assert_equal, all};
    use ndarray::Array2;

    use crate::core::model::{EmptyCellFunctions, NonEmptyCell};

    use super::*;

    #[test]
    fn x_wing_strategy_test() {
        let mut allowed_values: Vec<EmptyCell> = get_allowed_values();

        let index_1 = allowed_values.iter().position(|x| x.row == 3 && x.column == 0).unwrap();
        let index_2 = allowed_values.iter().position(|x| x.row == 6 && x.column == 5).unwrap();

        x_wing_strategy(&mut &mut allowed_values);

        assert_eq!(allowed_values[index_1].values, vec![3,4]);
        assert_eq!(allowed_values[index_2].values, vec![1,8]);
    }


    #[test]
    fn find_x_wing_pairs_rows_test() {
        let mut allowed_values: Vec<EmptyCell> = get_allowed_values();
        assert!(find_x_wing_pairs_rows(&mut allowed_values, 6, 4,8));

        let index_1 = allowed_values.iter().position(|x| x.row == 3 && x.column == 0).unwrap();
        let index_2 = allowed_values.iter().position(|x| x.row == 6 && x.column == 5).unwrap();

        assert_eq!(allowed_values[index_1].values, vec![3,4]);
        assert_eq!(allowed_values[index_2].values, vec![1,8]);
    }

    fn get_allowed_values () -> Vec<EmptyCell> {

        let mut allowed_values: Vec<EmptyCell> = Vec::new();

        allowed_values.push(EmptyCell::new(1, 5, vec![4, 5]));
        allowed_values.push(EmptyCell::new(2, 5, vec![2, 3]));
        allowed_values.push(EmptyCell::new(3, 0, vec![3, 4, 6]));
        allowed_values.push(EmptyCell::new(3, 5, vec![5,8]));
        allowed_values.push(EmptyCell::new(4, 0, vec![3,6]));
        allowed_values.push(EmptyCell::new(4, 5, vec![3,6]));
        allowed_values.push(EmptyCell::new(6, 5, vec![1,6,8]));
        allowed_values.push(EmptyCell::new(7, 5, vec![1,2,4]));
        allowed_values.push(EmptyCell::new(8, 0, vec![4,6]));
        allowed_values.push(EmptyCell::new(8, 5, vec![4,6]));

        return allowed_values;
    }
}
