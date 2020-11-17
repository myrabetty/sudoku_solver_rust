pub mod utilities {
    use std::collections::{hash_map::Entry, HashMap};
    use std::hash::Hash;
    use logs::debug;
    use crate::model::model::EmptyCell;

    pub fn iters_equal_anyorder<T: Eq + Hash>(i1: impl Iterator<Item=T>, i2: impl Iterator<Item=T>) -> bool {
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

    impl PartialEq for EmptyCell {
        fn eq(&self, other: &Self) -> bool {
            if self.row == other.row && self.column == other.column {
                debug!("for row {} and column {} the following values were compared: {:?} and {:?}", self.row, self.column, self.values, other.values);
                return iters_equal_anyorder(self.values.clone().into_iter(), other.values.clone().into_iter());
            }
            return false;
        }
    }
}
