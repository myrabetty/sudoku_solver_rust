use crate::initialize_grid::initialize_grid::{read_input_file, generate_grid, write_output_file};
mod initialize_grid;
use crate::validator::validator::{validate_grid};
mod validator;
use crate::solver::solver::{solve};
mod solver;

mod model;
mod initialize_empty_cells;
mod solver_helper;
mod utilities;

fn main(){

    let input_data = read_input_file("/Users/esemboloni/source/pocs/sudoku_solver/src/example_3.txt");
    let grid = generate_grid(input_data);
    validate_grid(&grid);
    let complete_grid = solve(grid);

    write_output_file(&complete_grid,"/Users/esemboloni/source/pocs/sudoku_solver/src/output.txt");
}
