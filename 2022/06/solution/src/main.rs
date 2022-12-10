use std::fs::File;
use std::io::{self, BufRead, BufReader};

use crate::helpers::{find_first_marker};

pub mod helpers;

fn main() -> io::Result<()> {
    // TEST BEGIN
    // let test_vec = vec![
    //     ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
    //     ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
    //     ("nppdvjthqldpwncqszvftbrmjlhg", 6),
    //     ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
    //     ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
    // ];
    
    // for (test_string, exepected_result) in test_vec {
    //     assert_eq!(find_first_marker(test_string.to_string(), 4), exepected_result);
    // }
    // TEST END

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut lines_iter = reader.lines().map(|line| line.unwrap());
    if let Some(input_line) = lines_iter.next() { // only need first line of input
        println!("Part one: {}", find_first_marker(&input_line, 4));
        println!("Part two: {}", find_first_marker(&input_line, 14));
    }

    Ok(())
}
