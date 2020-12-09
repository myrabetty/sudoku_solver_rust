use sudoku_solver::core::initialize_grid;
use sudoku_solver::core::initialize_grid::{read_input_file, generate_grid, write_output_file};
use sudoku_solver::core::solver::solve;
use sudoku_solver::core::validator::validate_grid;

fn main(){

    let input_data = read_input_file("/Users/esemboloni/source/pocs/sudoku_solver/samples/example_4.txt");
    let grid = generate_grid(input_data);
    validate_grid(&grid);
    let complete_grid = solve(grid);

    write_output_file(&complete_grid,"/Users/esemboloni/source/pocs/sudoku_solver/src/output.txt");
}
