use std::fs::File;
use std::io::{self, BufReader};

use crate::helpers::{InputRows, perform_procedures, perform_procedures_two};

pub mod helpers;

fn main() -> io::Result<()> {
    // TEST BEGIN
    // let test_file = File::open("small-input.txt")?;
    // let mut test_reader = BufReader::new(test_file);
    // let test_input_rows = InputRows {
    //     directions_start_line: 6,
    //     num_stacks: 3,
    //     row_line: 4
    // };
    // let test_result = perform_procedures(&mut test_reader, test_input_rows);
    // let expected_result = "CMZ";
    
    // assert_eq!(test_result, expected_result);
    // TEST END

    let file = File::open("input.txt")?;
    let mut reader = BufReader::new(file);
    let input_rows = InputRows {
        directions_start_line: 11,
        num_stacks: 9,
        row_line: 9
    };
    let result_one = perform_procedures(&mut reader, input_rows);
    let result_two = perform_procedures_two(&mut reader, input_rows);

    println!("Part one result: {}", result_one);
    println!("Part two result: {}", result_two);
    Ok(())
}
