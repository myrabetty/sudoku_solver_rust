use sudoku_solver::core::initialize_grid;
use sudoku_solver::core::initialize_grid::{read_input_file, generate_grid, map_request_to_grid};
use sudoku_solver::core::solver::solve;
use sudoku_solver::core::validator::validate_grid;
use sudoku_solver::template::template::show_sudoku_state;
use sudoku_solver::core::model::{EmptyCell, Guess};
use std::env;
use actix_web::{web, App, HttpServer, Result, Responder, HttpResponse};
use serde::Deserialize;
use actix_web::body::Body;
use actix_web::web::Json;
use log::debug;
use std::ops::Deref;


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

async fn get_solution(path: web::Path<(String,)>) -> HttpResponse {
    debug!("request is {:?}", path);
    let sudoku = include_str!("template/result.html");

    //for now we return this static result
    HttpResponse::Ok()
        .content_type("text/html")
        .body(sudoku)
    //HttpResponse::Ok().body("got the request all right")
    /*let grid = map_request_to_grid(&request.deref());
    validate_grid(&grid); //to do if grid is not valid return error.
    let complete_grid = solve(grid);*/

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    /*let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let input_data = read_input_file(filename);
    let grid = generate_grid(input_data);
    validate_grid(&grid);
    let complete_grid = solve(grid);
    show_sudoku_state(&complete_grid, &Vec::new());
    return Ok(());*/

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


