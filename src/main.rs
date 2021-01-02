use sudoku_solver::core::initialize_grid;
use sudoku_solver::core::initialize_grid::{generate_grid};
use sudoku_solver::core::solver::solve;
use sudoku_solver::core::validator::validate_grid;
use sudoku_solver::template::template::{show_sudoku_state_in_html};
use sudoku_solver::core::model::{EmptyCell, Guess, NonEmptyCell};
use sudoku_solver::core::solver_helper::{initialize_empty_values};
use std::env;
use actix_web::{web, App, HttpServer, Result, Responder, HttpResponse};
use serde::Deserialize;
use actix_web::body::Body;
use actix_web::web::Json;
use log::debug;
use std::str::Chars;
use std::borrow::Borrow;
use ndarray::Array2;


/// extract `Info` using serde
async fn index() -> HttpResponse {
    let sudoku = include_str!("template/init_sudoku.html");

    HttpResponse::Ok()
        .content_type("text/html")
        .body(sudoku)
}

/// extract `Info` using serde
async fn style_sheet() -> HttpResponse {
    let style = include_str!("template/mystyle.css");

    HttpResponse::Ok()
        .content_type("text/css")
        .body(style)
}

async fn get_solution(input: web::Path<String>) -> HttpResponse {
    debug!("request is {:?}", input);

    let input_numbers: Vec<char> = input.chars().collect();
    let grid: Array2<NonEmptyCell> = generate_grid(&input_numbers);
    validate_grid(&grid); //to do if grid is not valid return error.

    let solution = solve(grid);

    let result = match solution {
        Ok(final_grid) => {
            show_sudoku_state_in_html(&final_grid, &Vec::new())
        },
        Err(final_state) => {
            show_sudoku_state_in_html(&final_state.0, &final_state.1)
        }
    };

    return HttpResponse::Ok()
        .content_type("text/html")
        .body(result);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .route("/get_solution/{input}", web::get().to(get_solution))
            .route("mystyle.css", web::get().to(style_sheet))
            .route("/", web::get().to(index))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
}


