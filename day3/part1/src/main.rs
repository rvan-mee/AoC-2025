use std::error::Error;
use std::fs::File;
use std::env;
use std::io::{self, BufRead};
use std::path::Path;

fn get_largest_number(line: &String) -> u32 {
    let mut biggest_num = '0';
    let mut biggest_num_index = 0;
    for i in 0..(line.len()-1) {
        if line.chars().nth(i).expect("") > biggest_num {
            biggest_num = line.chars().nth(i).expect("");
            biggest_num_index = i;
        }
    }
    
    let mut second_biggest_num = '0';
    for i in biggest_num_index+1..line.len() {
        if line.chars().nth(i).expect("") > second_biggest_num {
            second_biggest_num = line.chars().nth(i).expect("");
        }
    }

    let mut res: u32 = 0;
    res += biggest_num.to_digit(10).expect("Unexpected character found") * 10;
    res += second_biggest_num.to_digit(10).expect("Unexpected character found");
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

    let mut res: u32 = 0;
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
