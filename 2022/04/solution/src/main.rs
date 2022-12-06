use std::fs::File;
use std::io::{self, BufReader};

use crate::helpers::{get_overlapping_pairs};

pub mod helpers;
pub mod types;

fn main() -> io::Result<()> {
    // TEST BEGIN
    let test_file = File::open("small-input.txt")?;
    let mut test_reader = BufReader::new(test_file);
    let (test_result_one, test_result_two) = get_overlapping_pairs(&mut test_reader);
    let expected_result_one = 2;
    let expected_result_two = 4;

    assert_eq!(test_result_one, expected_result_one);
    assert_eq!(test_result_two, expected_result_two);
    // TEST END

    let file = File::open("input.txt")?;
    let mut reader = BufReader::new(file);
    let (first_result, second_result) = get_overlapping_pairs(&mut reader);

    println!("Part one: {}", first_result);
    println!("Part two: {}", second_result);

    Ok(())
}
