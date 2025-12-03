use std::error::Error;
use std::fs::File;
use std::env;
use std::io::{self, BufRead};

const INPUT_ERROR: &'static str = "Error parsing input file";

fn digit_count(mut num: u64) -> u32 {
    let mut count = 1;
    while num >= 10 {
        num /= 10;
        count += 1;
    }
    count
}

fn is_valid_id(id: u64) -> bool {
    let amount_of_digits = digit_count(id);
    if amount_of_digits % 2 != 0 {
        return true;
    }

    let split_num = 10_u64.pow(amount_of_digits/2);
    if id%split_num == id/split_num {
        return false;
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
