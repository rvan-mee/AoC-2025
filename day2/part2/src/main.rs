use std::error::Error;
use std::fs::File;
use std::env;
use std::io::{self, BufRead};

const INPUT_ERROR: &'static str = "Error parsing input file";

fn is_valid_id(id: u64) -> bool {
    let id_as_str: String = id.to_string();
    let max_slice_size: usize = id_as_str.len() / 2;
    let mut curr_slice_size: usize = 1;

    while curr_slice_size <= max_slice_size {
        let mut curr_slice_pos: usize = curr_slice_size;
        let compare_slice: &str = &id_as_str[0..curr_slice_size];
        loop {
            // All digits of the string with this slice size repeat, found an invalid ID
            if curr_slice_pos == id_as_str.len() {
                return false;
            }
            else if curr_slice_pos + curr_slice_size > id_as_str.len() {
                return true;
            }

            if *compare_slice != id_as_str[curr_slice_pos..curr_slice_pos+curr_slice_size] {
                break ;
            }

            curr_slice_pos += curr_slice_size;
        }
        curr_slice_size += 1;
    }
    true
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err("Please provide an input file")?;
    }

    let mut result: u64 = 0;
    let file = File::open(&args[1]).expect(INPUT_ERROR);
    let mut buf_reader = io::BufReader::new(file);
    loop {
        let mut raw_input = vec![];
        let bytes_read = buf_reader.read_until(b',', &mut raw_input).expect(INPUT_ERROR);
        if bytes_read == 0 { break; }

        let input: String = String::from_utf8(raw_input).expect(INPUT_ERROR).trim_end_matches(',').trim_end_matches('\n').to_string();
        let delim_index = input.find('-').expect(INPUT_ERROR);

        let range_start = input[0..delim_index].parse::<u64>().expect(INPUT_ERROR);
        let range_end = input[delim_index+1..].parse::<u64>().expect(INPUT_ERROR);

        for to_check in range_start..=range_end {
            if is_valid_id(to_check) == false {
                result += to_check;
            }
        }
    }
    println!("Answer is {}", result);

    Ok(())
}
