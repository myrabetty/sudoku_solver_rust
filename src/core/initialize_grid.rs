use ndarray::Array2;
use crate::core::model::{NonEmptyCell, GridFunctions};

pub fn generate_grid(input_data: &Vec<char>) -> Array2<NonEmptyCell> {
    let mut grid: Array2<NonEmptyCell> = Array2::default((9, 9));
    grid.add_quadrants_information();

    let mut counter = 0;
    let mut row: usize = 0;
    let mut column: usize = 0;
    loop {
        if row == 9 { break; }
        loop {
            if column == 9 { break; }
            let a = *input_data.get(counter).unwrap();
            column = assign_value_and_increase_index(&mut grid, &mut row, &mut column, a).unwrap();
            counter = counter + 1;
        }
        row = row + 1;
        column = 0
    }

    println!("this is my grid: {:?}", grid);
    grid
}

fn assign_value_and_increase_index(grid: &mut Array2<NonEmptyCell>, i: &mut usize, j: &mut usize, a: char) -> Result<usize, String> {
    return if a >= '0' && a <= '9' {
        let b = a as u8 - '0' as u8;
        grid[[*i, *j]].value = b;
        Ok(*j + 1)
    } else {
        Err(format!("character {} not allowed", a))
    };
}


