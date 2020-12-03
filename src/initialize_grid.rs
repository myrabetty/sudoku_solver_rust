pub(crate) mod initialize_grid {
    use std::fs::File;
    use std::io::{Read, Write};
    use std::ops::Add;

    use ndarray::Array2;

    use crate::model::model::{GridFunctions, NonEmptyCell};

    pub fn write_output_file(grid: &Array2<NonEmptyCell>, filename: &str) {
        let mut file = File::create(filename).expect("file cannot be created");
        for i in 0..9 {
            let mut line = String::new();
            let mut index =0;
            grid.row(i).iter().for_each(|element| {
                line.push_str(element.value.to_string().as_str());
                match index {
                    2|5 => line.push_str(" "),
                    _ => {}
                }
                index = index+ 1;
            });
            line = line + "\n";
            match i {
                2|5 => line = line + "\n",
                _ => {}
            }
            file.write(line.as_bytes());
        }
    }


    pub fn read_input_file(filename: &str) -> Vec<char> {
        let mut file = File::open(filename).expect("File not found");
        let mut content = String::new();
        file.read_to_string(&mut content).expect("Content not found");
        content.chars().collect()
    }


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

    fn assign_value_and_increase_index(grid: &mut Array2<NonEmptyCell>, i: &mut usize, j: &mut usize, a: char) -> Result<usize, &'static str> {
        return if a >= '0' && a <= '9' {
            let b = a as u8 - '0' as u8;
            grid[[*i, *j]].value = b;
            Ok(*j + 1)
        } else if a == ' ' || a == '\n' || a == ',' {
            Ok(*j)
        } else {
            Err("character not allowed")
        };
    }
}


