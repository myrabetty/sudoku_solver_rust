#[cfg(test)]
mod tests {
    use handlebars::Handlebars;
    use serde_json::json;

    //```construct the current state of the sudoku given the current values and the unset possible values
    fn show_grid(){

    }

    #[test]
    fn display_template_test() {

        let data = &json!({
                "house": [
                    {"cell":[
                    {"value":1},
                    {"value":2},
                    {"value":3},
                    {"value":4},
                    {"value":5},
                    {"value":6},
                    {"value":7},
                    {"value":8},
                    {"pencil-marks":[1,2,4]}
                    ]},
                    //{"cell":[1,2,3,4,5,6,7,8,9]},
            ]
        });

        let mut reg = Handlebars::new();
        // render without register

        //reg.render_template("Hello {{name}}", &json!({"name": "foo"}));

        // register template using given name
        reg.register_template_file("tpl_1", "/Users/esemboloni/source/pocs/sudoku_solver/src/template/grid_template.hbs").unwrap();
        let result = reg.render("tpl_1", data);
        println!("{}",result.unwrap());
    }
}
