use std::io::{prelude::*, BufReader};

use crate::types::{Sections};

pub fn get_overlapping_pairs<R: Read>(reader: &mut BufReader<R>) -> (u32, u32) {
    let mut part_one_result: u32 = 0;
    let mut part_two_result: u32 = 0;

    for line in reader.lines() {
        if let Ok(line_val) = line {
            let pairs: Vec<&str> = line_val.split(",").collect();
            let first_pair = get_start_and_end(pairs[0]);
            let second_pair = get_start_and_end(pairs[1]);
            let first_sections = Sections::new(first_pair.0, first_pair.1);
            let second_sections = Sections::new(second_pair.0, second_pair.1);

            if fully_overlaps(&first_sections, &second_sections) {
                part_one_result += 1;
                part_two_result += 1;
            } else if partially_overlaps(&first_sections, &second_sections) {
                part_two_result += 1;
            }


        }
    }

    return (part_one_result, part_two_result);
}

fn fully_overlaps(&first_sections: &Sections, &second_sections: &Sections) -> bool {
    (first_sections.start <= second_sections.start) && (first_sections.end >= second_sections.end) ||
        (second_sections.start <= first_sections.start) && (second_sections.end >= first_sections.end)
}

fn partially_overlaps(&first_sections: &Sections, &second_sections: &Sections) -> bool {
    (first_sections.start >= second_sections.start) && (first_sections.start <= second_sections.end) ||
        (first_sections.end >= second_sections.start) && (first_sections.end <= second_sections.end)
}

fn get_start_and_end(pair: &str) -> (u32, u32) {
    let split: Vec<&str> = pair.split("-").collect();
    let start = split[0].parse::<u32>().unwrap();
    let end = split[1].parse::<u32>().unwrap();

    (start, end)
}