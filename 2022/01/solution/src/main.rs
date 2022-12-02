use std::cmp;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::num::ParseIntError;

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let mut reader = BufReader::new(file);
    let max:i32 = get_max_elf(&mut reader).unwrap();

    let second_file = File::open("input.txt")?;
    let mut second_reader = BufReader::new(second_file);
    let top_three_sum:i32 = get_top_num_elves(&mut second_reader, 3).unwrap();

    println!("Max: {}", max);
    println!("Top 3 sum: {}", top_three_sum);
    Ok(())
}

fn get_max_elf<R: Read>(reader: &mut BufReader<R>) -> Result<i32, std::io::Error> {
    let mut current_max: i32 = 0;
    let mut current_sum: i32 = 0;

    for line in reader.lines() {
        let result: Result<i32, ParseIntError> = line?.trim_end().parse();
        
        if result.is_ok() { // is a number
            current_sum = current_sum + result.unwrap();
        } else { // is a new line
            current_max = cmp::max(current_max, current_sum);
            current_sum = 0;
        }
    }

    // one final comparison in case last elf is larger than current max and return
    return Ok(cmp::max(current_max, current_sum));
}

fn get_top_num_elves<R: Read>(reader: &mut BufReader<R>, end: usize) -> Result<i32, std::io::Error> {
    let mut current_sum: i32 = 0;
    let mut sum_vec: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let result: Result<i32, ParseIntError> = line?.trim_end().parse();
        
        if result.is_ok() { // is a number
            current_sum = current_sum + result.unwrap();
        } else { // is a new line
            sum_vec.push(current_sum);
            current_sum = 0;
        }
    }

    sum_vec.sort_by(|a, b| b.cmp(a));
    
    let mut result = 0;

    for i in 0..end {
        result = result + sum_vec[i];
    }

    return Ok(result);
}