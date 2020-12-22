use crate::core::model::EmptyCell;
use log::debug;

pub fn naked_pairs_strategy(mut allowed_values: &mut &mut Vec<EmptyCell>) {
    for value_1 in 1..10_u8 {
        for value_2 in (value_1 + 1)..10_u8 {
            for index in 0..9_usize {
                match find_connected_naked_pairs(&mut allowed_values, value_1, value_2, index, |x: &EmptyCell, index| x.row == index) {
                    true => {
                        debug!("Found naked pair {:} {:} in row {:}", value_1, value_2, index);
                    }
                    _ => {}
                }
                match find_connected_naked_pairs(&mut allowed_values, value_1, value_2, index, |x: &EmptyCell, index| x.column == index) {
                    true => {
                        debug!("Found naked pair {:} {:} in column {:}", value_1, value_2, index);
                    }
                    _ => {}
                }

                match find_connected_naked_pairs(&mut allowed_values, value_1, value_2, index, |x: &EmptyCell, index| x.quadrant == index) {
                    true => {
                        debug!("Found naked pair {:} {:} in quadrant {:}", value_1, value_2, index);
                    }
                    _ => {}
                }
            }
        }
    }
}

// finds if there are two connected cells that contain the same pair.If such cells exists removes these values from all other connected cells.
fn find_connected_naked_pairs(allowed_values: &mut Vec<EmptyCell>, value_1: u8, value_2: u8, index: usize, filter: fn(&EmptyCell, usize) -> bool) -> bool {
    let number_cells_with_couple_only = allowed_values
        .iter()
        .filter(|x| filter(x, index) && x.values.contains(&value_1) && x.values.contains(&value_2) && x.values.len() == 2).count();
    if number_cells_with_couple_only == 2 {
        allowed_values.iter_mut().for_each(|x|
            {
                if filter(x, index) && !(x.values.contains(&value_1) && x.values.contains(&value_2) && x.values.len() == 2) {
                    let new_values: Vec<u8> = x.values.clone().into_iter().filter(|&value| value != value_1 && value != value_2).collect();
                    x.values = new_values;
                }
            });
        return true;
    }
    return false;
}
