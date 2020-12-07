#[cfg(test)]
mod tests {
    use handlebars::Handlebars;
    use serde_json::json;
    #[test]
    fn display_template() {

        let data = &json!({
            "house_row": [
                {"house": [
                    {"cell":
                        {"value":1}
                    },
                    {"cell":
                        {"value":1}
                    },
                    {"cell":
                        {"value":1}
                    },
                ]},
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
