use std::fs::File;
use std::io::Write;

use handlebars::Handlebars;
use logs::debug;
use ndarray::Array2;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::core::model::{EmptyCell, NonEmptyCell};
use crate::core::utilities::{get_index_in_quadrant, get_quadrant};

#[derive(Deserialize, Serialize, Default, Debug)]
struct Sudoku {
    quadrants: [[Cell; 9]; 9]
}

#[derive(Deserialize, Serialize, Default, Debug)]
struct Cell {
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pencil_marks: Option<Vec<u8>>,
}


//```construct the current state of the sudoku given the current values and the unset possible values
fn show_sudoku_state(grid: &Array2<NonEmptyCell>, allowed_values: &Vec<EmptyCell>) {
    let mut reg = Handlebars::new();
    let mut sudoku = Sudoku::default();
    reg.register_template_file("template", "/Users/esemboloni/source/pocs/sudoku_solver/src/template/grid_template.hbs").unwrap();

    for i in 0..9 {
        for j in 0..9 {
            let quadrant_index = get_index_in_quadrant(i, j);
            let quadrant = get_quadrant(i, j);
            if grid[[i, j]].value == 0 {
                sudoku.quadrants[quadrant][quadrant_index].value = Some(grid[[i, j]].value);
            } else {
                let index = allowed_values.iter().position(|x| x.row == i && x.column == j).unwrap();
                sudoku.quadrants[quadrant][quadrant_index].pencil_marks = Some(allowed_values[index].clone().values);
            }
        }
    }

    let result = reg.render("template", &json!(sudoku)).unwrap();
    let mut file = File::create("/Users/esemboloni/source/pocs/sudoku_solver/src/template/result.html").unwrap();
    file.write(result.as_bytes());
}

#[cfg(test)]
mod tests {
    use itertools::assert_equal;

    use crate::core::model::EmptyCellFunctions;

    use super::*;

    #[test]
    fn map_grid_test() {
        let mut sudoku = Sudoku::default();
        debug!("{:?}", sudoku);
    }

    #[test]
    fn build_json_test() {
        let data = &json!(
                 [
                    {"value":1},
                    {"value":2},
                    {"value":3},
                    {"value":4},
                    {"value":5},
                    {"value":6},
                    {"value":7},
                    {"value":8},
                    {"pencil_marks":[1,2,4]}
                  ]
        );

        let quadrant: [Cell; 9] = [
            Cell { value: Some(1), pencil_marks: None },
            Cell { value: Some(2), pencil_marks: None },
            Cell { value: Some(3), pencil_marks: None },
            Cell { value: Some(4), pencil_marks: None },
            Cell { value: Some(5), pencil_marks: None },
            Cell { value: Some(6), pencil_marks: None },
            Cell { value: Some(7), pencil_marks: None },
            Cell { value: Some(8), pencil_marks: None },
            Cell { value: None, pencil_marks: Some(vec![1, 2, 4]) }
        ];

        assert_eq!(serde_json::to_string(&json!(quadrant)).unwrap(), serde_json::to_string(data).unwrap());
    }

    #[test]
    fn display_current_grid_state_test() {
        let mut grid: Array2<NonEmptyCell> = Array2::default((9, 9));
        grid[[0, 0]].value = 4;
        grid[[0, 2]].value = 6;
        grid[[1, 0]].value = 2;
        grid[[1, 2]].value = 8;
        grid[[1, 3]].value = 7;
        grid[[1, 4]].value = 4;
        grid[[1, 6]].value = 3;
        grid[[1, 7]].value = 9;
        grid[[2, 2]].value = 3;
        grid[[2, 4]].value = 1;
        grid[[2, 7]].value = 8;
        grid[[2, 8]].value = 4;
        grid[[3, 1]].value = 6;
        grid[[3, 5]].value = 8;
        grid[[3, 6]].value = 4;
        grid[[4, 1]].value = 4;
        grid[[4, 2]].value = 8;
        grid[[4, 3]].value = 4;
        grid[[4, 4]].value = 2;
        grid[[4, 6]].value = 7;
        grid[[4, 7]].value = 6;
        grid[[4, 8]].value = 3;
        grid[[5, 2]].value = 4;
        grid[[5, 6]].value = 9;
        grid[[5, 8]].value = 8;
        grid[[6, 1]].value = 3;
        grid[[6, 5]].value = 4;
        grid[[7, 0]].value = 6;
        grid[[7, 1]].value = 4;
        grid[[7, 3]].value = 2;
        grid[[7, 6]].value = 8;
        grid[[7, 7]].value = 3;
        grid[[8, 5]].value = 3;
        grid[[8, 6]].value = 5;
        grid[[8, 7]].value = 4;
        grid[[8, 8]].value = 9;

        let mut allowed_values: Vec<EmptyCell>;

        allowed_values.push(EmptyCell::new(0, 1, vec![1, 5]));
        allowed_values.push(EmptyCell::new(0, 3, vec![3, 8]));
        allowed_values.push(EmptyCell::new(0, 4, vec![3, 8]));
        allowed_values.push(EmptyCell::new(0, 5, vec![2, 5,9]));
        allowed_values.push(EmptyCell::new(0, 6, vec![1,2]));
        allowed_values.push(EmptyCell::new(0, 7, vec![1,2,5,7]));
        allowed_values.push(EmptyCell::new(0, 8, vec![1,2,5,7]));
        allowed_values.push(EmptyCell::new(1, 0, vec![1,5]));
        allowed_values.push(EmptyCell::new(1, 5, vec![5,6]));
        allowed_values.push(EmptyCell::new(1, 8, vec![1,5,6]));
        allowed_values.push(EmptyCell::new(2, 0, vec![5,7,9]));
        allowed_values.push(EmptyCell::new(2, 1, vec![5,7,9]));
        allowed_values.push(EmptyCell::new(2, 4, vec![5,6,9]));
        allowed_values.push(EmptyCell::new(2, 6, vec![2,6]));
        allowed_values.push(EmptyCell::new(3, 0, vec![3,5,7,9]));
        allowed_values.push(EmptyCell::new(3, 2, vec![2,5,7,9]));
        allowed_values.push(EmptyCell::new(3, 3, vec![3,5,9]));
        allowed_values.push(EmptyCell::new(3, 4, vec![1,3,5,7,9]));
        allowed_values.push(EmptyCell::new(3, 7, vec![1,2,5]));
        allowed_values.push(EmptyCell::new(3, 8, vec![1,2,5]));
        allowed_values.push(EmptyCell::new(4, 2, vec![5,9]));
        allowed_values.push(EmptyCell::new(4, 5, vec![5,9]));
        allowed_values.push(EmptyCell::new(5, 0, vec![3,5,7]));
        allowed_values.push(EmptyCell::new(5, 1, vec![2,5,7]));
        allowed_values.push(EmptyCell::new(5, 3, vec![3,5,6]));
        allowed_values.push(EmptyCell::new(5, 4, vec![1,3,5,6,7]));
        allowed_values.push(EmptyCell::new(5, 7, vec![1,2,5]));
        allowed_values.push(EmptyCell::new(6, 0, vec![5,7,8,9]));
        allowed_values.push(EmptyCell::new(6, 2, vec![1,5,7,9]));
        allowed_values.push(EmptyCell::new(6, 3, vec![5,8,9]));
        allowed_values.push(EmptyCell::new(6, 4, vec![1,5,7,8,9]));
        allowed_values.push(EmptyCell::new(6, 6, vec![1,2,6]));
        allowed_values.push(EmptyCell::new(6, 7, vec![1,2,7]));
        allowed_values.push(EmptyCell::new(6, 8, vec![1,2,6,7]));
        allowed_values.push(EmptyCell::new(7, 2, vec![5,9]));
        allowed_values.push(EmptyCell::new(7, 4, vec![5,9]));
        allowed_values.push(EmptyCell::new(7, 8, vec![1,7]));
        allowed_values.push(EmptyCell::new(8, 0, vec![7,8]));
        allowed_values.push(EmptyCell::new(8, 1, vec![2,7]));
        allowed_values.push(EmptyCell::new(8, 2, vec![1,2,7]));
        allowed_values.push(EmptyCell::new(8, 3, vec![6,8]));
        allowed_values.push(EmptyCell::new(8, 4, vec![1,6,7,8]));
    }


    #[test]
    fn show_sudoku_state_test() {
        let data = &json!({
                "quadrants": [
                [
                    {"value":1},
                    {"value":2},
                    {"value":3},
                    {"value":4},
                    {"value":5},
                    {"value":3},
                    {"value":7},
                    {"value":8},
                    {"pencil_marks":[1,2,4]}
                    ],
                    [
                    {"value":1},
                    {"value":2},
                    {"value":3},
                    {"value":4},
                    {"value":5},
                    {"value":3},
                    {"value":7},
                    {"value":8},
                    {"pencil_marks":[1,2,4]}
                    ],
                 ]
        });

        let mut reg = Handlebars::new();
        reg.register_template_file("tpl_1", "/Users/esemboloni/source/pocs/sudoku_solver/src/template/grid_template.hbs").unwrap();
        let result = reg.render("tpl_1", data).unwrap();
    }
}
