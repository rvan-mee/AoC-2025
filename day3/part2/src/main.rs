use std::error::Error;
use std::fs::File;
use std::env;
use std::io::{self, BufRead};
use std::path::Path;

fn get_largest_number(line: &String) -> u64 {
    let mut char_buf = vec!['0'; 12];
    let mut last_biggest_index = 0;

    for current_buf_index in 0..12 {
        for i in last_biggest_index..(line.len()-11+current_buf_index) {
            if line.chars().nth(i).expect("") > char_buf[current_buf_index] {
                char_buf[current_buf_index] = line.chars().nth(i).expect("");
                last_biggest_index = i + 1;
            }
        }
    }

    let mut res = 0;
    for i in 0..12 {
        res += char_buf[i].to_digit(10).expect("Unexpected character found") as u64 * 10_u64.pow(11-(i as u32));
    }
    res
}

fn get_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err("Please provide an input file")?;
    }

    let mut res: u64 = 0;
    match get_lines(&args[1]) {
        Ok(lines) => {
            for line in lines.map_while(Result::ok) {
                res += get_largest_number(&line);
            }
        }
        Err(e) => {
            return Err(format!("Unable to read file: {}", e.to_string()))?;
        }
    }

    println!("Answer is: {}", res);

    Ok(())
}
