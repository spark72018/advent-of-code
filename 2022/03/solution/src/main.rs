use std::fs::File;
use std::io::{self, BufReader, Seek, SeekFrom};

use crate::helpers::{
    compute_total_priority_sum_part_one, 
    compute_total_priority_sum_part_two, 
    get_priority_map
};

pub mod helpers;

fn main() -> io::Result<()> {
    // TEST BEGIN
    // let test_file = File::open("small-input.txt")?;
    // let mut test_reader = BufReader::new(test_file);
    // let test_total_priority_sum = compute_total_priority_sum(
    //     &mut test_reader,
    //     get_priority_map()
    // );
    // let expected_test_sum = 59;

    // assert_eq!(test_total_priority_sum, expected_test_sum);
    // TEST END

    let file = File::open("input.txt")?;
    let mut reader = BufReader::new(file);
    let total_priority_sum_part_one = compute_total_priority_sum_part_one(
        &mut reader,
        get_priority_map()
    );

    reader.seek(SeekFrom::Start(0));

    let total_priority_sum_part_two = compute_total_priority_sum_part_two(
        &mut reader,
        get_priority_map()
    );

    println!("Part one: {}", total_priority_sum_part_one);
    println!("Part two: {}", total_priority_sum_part_two);

    Ok(())
}
