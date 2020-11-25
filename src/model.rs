pub mod model {
    use std::collections::HashMap;

    use logs::debug;
    use ndarray::Array2;

    use crate::utilities::utilities::get_quadrant_position;
    use crate::validator::validator::validate_grid;

    // ```it describes a cell position and its value
    #[derive(Debug, Default, Clone)]
    pub struct NonEmptyCell {
        pub value: u8,
        pub quadrant: usize,
    }

    //```it describes an empty cell position and the values that can be in that cell.
    #[derive(Debug, Default, Clone)]
    pub struct EmptyCell {
        pub row: usize,
        pub column: usize,
        pub quadrant: usize,
        pub values: Vec<u8>,
    }

    #[derive(Debug, Default, Clone)]
    pub struct Guess {
        pub row: usize,
        pub column: usize,
        pub value: u8,
    }


    pub trait GridFunctions {
        // ``` add quadrant information to a grid
        fn add_quadrants_information(&mut self);
        // ```check if the grid is complete
        fn is_complete(&self) -> bool;
    }

    pub trait EmptyCellFunctions {
        fn create(row: usize, column: usize) -> Self;
        fn new(row: usize, column: usize, values: Vec<u8>) -> Self;
    }

    impl GridFunctions for Array2<NonEmptyCell> {
        fn add_quadrants_information(&mut self) {
            for i in 0..9 {
                for j in 0..9 {
                    self[[i, j]].quadrant = get_quadrant_position(i, j);
                    debug!("row = {}, column = {}, and quadrant {}", i, j, self[[i, j]].quadrant);
                }
            }
        }

        fn is_complete(&self) -> bool {
            return self.iter().filter(|&x| x.value == 0).count() == 0;
        }
    }

    impl EmptyCellFunctions for EmptyCell {
        fn create(row: usize, column: usize) -> Self {
            return EmptyCell {
                row,
                column,
                quadrant: get_quadrant_position(row, column),
                values: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            };
        }
        fn new(row: usize, column: usize, values: Vec<u8>) -> Self {
            return EmptyCell {
                row,
                column,
                quadrant: get_quadrant_position(row, column),
                values,
            };
        }
    }

    //todo move all this stuff. Too much logic this is not a model.
    // ```sets in the grid all Allowed values with only one possible value

    /*pub trait AllowedValuesFunctions {
    //```eliminates guessed_value as possible value from all other cells.
    fn eliminate_guessed_value_from_other_cells(&self, guessed_value: &GuessCell) -> Vec<EmptyCell>;

    //```returns all cells that have only one possible value after selecting guessed_value
    fn elimination_process(&self, guessed_value: &GuessCell) -> Vec<GuessCell>;

    // ```returns an hashmap with keys values and entries an array containing all cells which allows for that value
    //fn returns_hashmap_with_counts(&self) -> HashMap<u8, Vec<GuessedValue>>;
}*/

    /*pub trait GuessedValuesFunctions {
        //sets all guessed values in the grid.
        fn set_guessed_values_in_grid(&self, grid: &mut Array2<NonEmptyCell>);
    }

    pub trait GuessedValueFunctions {
        //sets a guessed value in the grid.
        fn set_guessed_value_in_grid(&self, grid: &mut Array2<NonEmptyCell>);
    }*/
    /*impl AllowedValuesFunctions for Vec<EmptyCell> {
        fn eliminate_guessed_value_from_other_cells(&self, guessed_value: &GuessCell) -> Vec<EmptyCell> {
            let mut new_guesses = self.clone();
            new_guesses.iter_mut().for_each(|x|

                if x.row == guessed_value.row && x.column == guessed_value.column {
                    x.values.retain(|&y| y != guessed_value.value)
                });

            return new_guesses;
        }

        fn elimination_process(&self, possible_guess: &GuessCell) -> Vec<GuessCell> {
            let mut new_guesses = self.clone();

            new_guesses.iter_mut().for_each(|mut x| {
                if x.row == possible_guess.row
                    || x.column == possible_guess.column
                    || x.quadrant == get_quadrant_position(possible_guess.row, possible_guess.column)
                {
                    x.values.retain(|&y| y != possible_guess.value)
                }
            });

            return new_guesses.iter().filter(|&allowed_values| allowed_values.values.len() == 1).map(|x| {
                return GuessCell {
                    row: x.row,
                    column: x.column,
                    value: *x.values.get(0).unwrap(),
                };
            }).collect();
        }

        // ``` returns hash-map with number and cells where the value could appear. The map is ordered
        // so that the numbers with smallest set of possibility comes first.
        /*fn returns_hashmap_with_counts(&self) -> HashMap<u8, Vec<GuessedValue>> {
            let mut counter: HashMap<u8, Vec<GuessedValue>> = HashMap::new();

            (1..9).map(|i| {
                let number_of_appearances: Vec<GuessedValue> = self.iter()
                    .filter(|guess| guess.values.contains(&i))
                    .map(|x| return GuessedValue {
                        row: x.row,
                        column: x.column,
                        value: i,
                    }).collect();
                if !number_of_appearances.is_empty() { counter.insert(i, number_of_appearances) };
            });
            return counter;
        }*/
    }*/

    /*impl GuessedValuesFunctions for Vec<GuessCell> {
        fn set_guessed_values_in_grid(&self, mut grid: &mut Array2<NonEmptyCell>) {
            self.iter().for_each(|guessed_value| guessed_value.set_guessed_value_in_grid(&mut grid));
        }
    }

    impl GuessedValueFunctions for GuessCell {
        fn set_guessed_value_in_grid(&self, grid: &mut Array2<NonEmptyCell>) {
            let i = self.row;
            let j = self.column;
            if grid[[i, j]].value != 0 { panic!("Something is seriously wrong") }
            grid[[i, j]].value = self.value;
        }
    }*/


    #[cfg(test)]
    mod tests {
        use super::*;

        /*#[test]
                        fn set_guessed_values_in_grid_test() {
                            let mut guessed_values: Vec<GuessCell> = Vec::new();
                            guessed_values.push(GuessCell {
                                row: 0,
                                column: 0,
                                value: 5,
                            });

                            guessed_values.push(GuessCell {
                                row: 5,
                                column: 2,
                                value: 8,
                            });

                            let mut grid: Array2<NonEmptyCell> = Array2::default((9, 9));
                            grid.add_quadrants_information();
                            guessed_values.set_guessed_values_in_grid(&mut grid);

                            assert_eq!(grid[[0, 0]].value, 5);
                            assert_eq!(grid[[5, 2]].value, 8);
                        }

                        #[test]
                        fn eliminate_guessed_value_from_other_cells() {
                            let mut guessed_values: Vec<GuessCell> = Vec::new();
                            guessed_values.push(GuessCell {
                                row: 5,
                                column: 2,
                                value: 8,
                            });
                            let mut allowed_values: Vec<EmptyCell> = Vec::new();
                            allowed_values.push(EmptyCell::create(5, 1));
                            allowed_values.push(EmptyCell::create(1, 2));
                            allowed_values.push(EmptyCell::create(8, 8));
                            allowed_values.push(EmptyCell::create(4, 1));

                            //assert_eq!()
                        }

                        #[test]
                        fn elimination_process_test() {}*/
    }
}

