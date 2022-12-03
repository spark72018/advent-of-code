use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader};

use crate::helpers::{
    compute_total_points, 
    get_hand_points_map,
    get_result_points_map
};
use crate::types::{Hand, Result};

pub mod helpers;
pub mod types;

fn main() -> io::Result<()> {
    // TEST BEGIN
    let test_file = File::open("small-input.txt")?;
    let mut test_reader = BufReader::new(test_file);
    let (test_total_points_one, test_total_points_two) = compute_total_points(
        &mut test_reader, 
        get_hand_points_map(), 
        get_result_points_map()
    );

    let expected_test_total_points_one = 25;
    let expected_test_total_points_two = 16;

    assert_eq!(test_total_points_one, expected_test_total_points_one);
    assert_eq!(test_total_points_two, expected_test_total_points_two);
    // TEST END

    // real puzzle input
    let file = File::open("input.txt")?;
    let mut reader = BufReader::new(file);
    let hand_points_map: HashMap<Hand, i32> = get_hand_points_map();
    let result_points_map: HashMap<Result, i32> = get_result_points_map();

    let (total_points_one, total_points_two) = compute_total_points(&mut reader, hand_points_map, result_points_map);

    println!("Total points part one: {}", total_points_one);
    println!("Total points part two: {}", total_points_two);
    
    Ok(())
}
