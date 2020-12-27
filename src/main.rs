use sudoku_solver::core::initialize_grid;
use sudoku_solver::core::initialize_grid::{read_input_file, generate_grid, write_output_file};
use sudoku_solver::core::solver::solve;
use sudoku_solver::core::validator::validate_grid;
use sudoku_solver::template::template::show_sudoku_state;
use sudoku_solver::core::model::EmptyCell;
use std::env;

fn main(){
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let input_data = read_input_file(filename);
    let grid = generate_grid(input_data);
    validate_grid(&grid);
    let complete_grid = solve(grid);
    show_sudoku_state(&complete_grid, &Vec::new());

}

#[cfg(test)]
mod tests {
    use super::*;
}


