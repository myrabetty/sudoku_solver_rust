use crate::core::model::{EmptyCell, Guess, EmptyCellFunctions};
use log::debug;
use itertools::Itertools;

pub fn apply_simple_coloring(guesses: &Vec<EmptyCell>) -> Option<Guess> {
    for value in 1..10_u8 {
        let mut pairs: Vec<EmptyCell> = guesses.clone().into_iter().filter(|guess| guess.values.len() == 2 && guess.values.contains(&value)).collect();
        let mut connected_length = 0;
        if pairs.len() > 1 {
            //let new_connected_element_found = false;
            let mut selected_element_index: Option<usize> = Some(0);
            while selected_element_index.is_some() {
                let position = selected_element_index.unwrap();
                let selected_element = pairs[position].clone();
                pairs.remove(position);
                connected_length = connected_length + 1;

                debug!("element for simple coloring {:?}", selected_element);
                selected_element_index = find_new_connect_element(&selected_element, &pairs);

            }
            debug!("connected length for value {:} is {:}", value, connected_length);
        }
    }
    return None;
}

// ```returns the position of the first connected element to selected_element given a candidate_pairs vector
fn find_new_connect_element(selected_element: &EmptyCell, candidate_pairs: &Vec<EmptyCell>) -> Option<usize> {
    return candidate_pairs.iter().position(|element| selected_element.is_connected(element));
}