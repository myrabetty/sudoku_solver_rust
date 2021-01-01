use crate::core::model::{EmptyCell, Guess, EmptyCellFunctions};
use log::debug;
use itertools::{Itertools, chain};

pub fn apply_simple_coloring(guesses: &Vec<EmptyCell>) -> Option<Guess> {
    let mut selected_element: EmptyCell = Default::default();
    let mut connected_length = 0;
    let mut chain: Chain = Chain {
        init_cell: Default::default(),
        length: 0,
        value: 0,
    };

    for value in 1..10_u8 {
        let mut pairs: Vec<EmptyCell> = guesses.clone().into_iter().filter(|guess| guess.values.len() == 2 && guess.values.contains(&value)).collect();

        while pairs.len() > 1 {
            connected_length = 0;
            let mut selected_element_index: Option<usize> = Some(0);
            let init_cell = pairs[0].clone();
            while selected_element_index.is_some() {
                let position = selected_element_index.unwrap();
                selected_element = pairs[position].clone();
                pairs.remove(position);
                connected_length = connected_length + 1;

                debug!("element for simple coloring {:?}", selected_element);
                selected_element_index = find_new_connect_element(&selected_element, &pairs);
            }
            if connected_length > chain.length {
                chain.init_cell = init_cell;
                chain.value = value;
                chain.length = connected_length;
            }
            debug!("connected length for value {:} is {:}", value, connected_length);
        }
    }

    return match chain.length > 2 {
        true => {
            debug!("simple coloring for value {:} is a chain of length {:}", chain.value, chain.length);
            Some(Guess { row: chain.init_cell.row, column: chain.init_cell.column, value: chain.value })
        }
        _ => {
            debug!("no simple colouring detected");
            None
        }
    };

}

// ```returns the position of the first connected element to selected_element given a candidate_pairs vector
fn find_new_connect_element(selected_element: &EmptyCell, candidate_pairs: &Vec<EmptyCell>) -> Option<usize> {
    return candidate_pairs.iter().position(|element| selected_element.is_connected(element));
}


struct Chain {
    pub init_cell: EmptyCell,
    pub length: usize,
    pub value: u8,
}