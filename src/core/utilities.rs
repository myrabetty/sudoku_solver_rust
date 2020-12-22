use std::collections::{hash_map::Entry, HashMap};
use std::hash::Hash;

use log::debug;

use crate::core::model::EmptyCell;

//``` returns the quadrant value given the row and the column.
pub fn get_quadrant(i: usize, j: usize) -> usize {
    return 3 * (i / 3) + j / 3;
}

//``` the position in the quadrant
pub fn get_index_in_quadrant(i: usize, j: usize) -> usize {
    return 3 * (i % 3) + (j % 3);
}


// ```return true if iterators contain same elements in any order.
pub fn iters_equal_any_order<T: Eq + Hash>(i1: impl Iterator<Item=T>, i2: impl Iterator<Item=T>) -> bool {
    fn get_lookup<T: Eq + Hash>(iter: impl Iterator<Item=T>) -> HashMap<T, usize> {
        let mut lookup = HashMap::<T, usize>::new();
        for value in iter {
            match lookup.entry(value) {
                Entry::Occupied(entry) => { *entry.into_mut() += 1; }
                Entry::Vacant(entry) => { entry.insert(0); }
            }
        }
        lookup
    }
    get_lookup(i1) == get_lookup(i2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_quadrant_position_test() {
        //``` maps row 0, columns 0 into 0
        assert_eq!(get_quadrant(0, 0), 0);
        //``` maps row 8, columns 0 into 6
        assert_eq!(get_quadrant(8, 0), 6);
        //``` maps row 8, columns 8 into 8
        assert_eq!(get_quadrant(8, 8), 8);
        //``` maps row 0, columns 8 into 2
        assert_eq!(get_quadrant(0, 8), 2);
        //``` maps row 4, columns 2 into 3
        assert_eq!(get_quadrant(4, 2), 3);
        //``` maps row 4, columns 7 into 4
        assert_eq!(get_quadrant(4, 7), 5);
    }


    #[test]
    fn get_position_in_quadrant_test() {
        //``` maps row 0, columns 0 into 0
        assert_eq!(get_index_in_quadrant(0, 0), 0);
        //``` maps row 1, columns 4 into 4
        assert_eq!(get_index_in_quadrant(1, 4), 4);
        //``` maps row 8, columns 0 into 6
        assert_eq!(get_index_in_quadrant(8, 0), 6);
        //``` maps row 8, columns 8 into 8
        assert_eq!(get_index_in_quadrant(8, 8), 8);
        //``` maps row 0, columns 8 into 2
        assert_eq!(get_index_in_quadrant(0, 8), 2);
        //``` maps row 4, columns 2 into 5
        assert_eq!(get_index_in_quadrant(4, 2), 5);
        //``` maps row 4, columns 7 into 4
        assert_eq!(get_index_in_quadrant(4, 7), 4);
    }
}
