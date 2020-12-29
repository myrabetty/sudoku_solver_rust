use std::fs::File;
use std::io::{Read, Write};

use ndarray::Array2;

use crate::core::model::{NonEmptyCell, GridFunctions, Guess};


pub fn map_request_to_grid(input_values: &Vec<Guess>) -> Array2<NonEmptyCell> {
    let mut grid: Array2<NonEmptyCell> = Array2::default((9, 9));
    input_values.iter().for_each(|input_value| grid[[input_value.row, input_value.column]].value = input_value.value);
    grid.add_quadrants_information();
    return grid;
}

// this is going to be used only for tenting purposes until I change the input to a json file
pub fn read_input_file(filename: &str) -> Vec<char> {
    let mut file = File::open(filename).expect("File not found");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Content not found");
    content.chars().collect()
}

// this is going to be used only for testing purposes maybe??
pub fn generate_grid(input_data: Vec<char>) -> Array2<NonEmptyCell> {
    let mut grid: Array2<NonEmptyCell> = Array2::default((9, 9));
    grid.add_quadrants_information();

    let mut counter = 0;
    let mut i: usize = 0;
    let mut j: usize = 0;
    loop {
        if i == 9 { break; }
        loop {
            if j == 9 { break; }
            let a = *input_data.get(counter).unwrap();
            j = assign_value_and_increase_index(&mut grid, &mut i, &mut j, a).unwrap();
            counter = counter + 1;
        }
        i = i + 1;
        j = 0
    }

    println!("this is my grid: {:?}", grid);
    grid
}

fn assign_value_and_increase_index(grid: &mut Array2<NonEmptyCell>, i: &mut usize, j: &mut usize, a: char) -> Result<usize, String> {
    return if a >= '0' && a <= '9' {
        let b = a as u8 - '0' as u8;
        grid[[*i, *j]].value = b;
        Ok(*j + 1)
    } else if a == ' ' || a == '\n' || a == ',' || a == '\r' {
        Ok(*j)
    } else {
        Err(format!("character {} not allowed", a))
    };
}


