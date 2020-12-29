use sudoku_solver::core::initialize_grid;
use sudoku_solver::core::initialize_grid::{read_input_file, generate_grid, write_output_file};
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

#[derive(Deserialize)]
struct Info {
    username: String,
}

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

async fn grid_send(request: Json<Vec<Guess>>) -> impl Responder {
    debug!("request is {:?}", request);
    HttpResponse::Ok().body("got the request all right")
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
            .route("/grid_send", web::post().to(grid_send))
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


