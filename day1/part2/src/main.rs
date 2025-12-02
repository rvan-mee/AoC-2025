use std::error::Error;
use std::fs::File;
use std::env;
use std::io::{self, BufRead};
use std::path::Path;

/**
 * returns 1 if right turn, -1 on left turn
 */
fn parse_turn_direction(direction: char) -> Result<i32, String> {
    if direction == 'L' {
        Ok(-1)
    } else if direction == 'R' {
        Ok(1)
    } else {
        Err(format!("Invalid direction: {}", direction))
    }
}

fn turn_dial(dial_state: &mut i32, zero_count: &mut i32, turn_distance: i32, turn_direction: i32)
{
    // For every full rotation we can add 1 'pass by zero'
    *zero_count += turn_distance / 100;
    
    if turn_direction == 1 {
        if (turn_distance % 100) >= 100 - *dial_state {
            *zero_count += 1;
        }
        *dial_state = *dial_state + (turn_distance * turn_direction % 100);
    } else if turn_direction == -1 {
        if (turn_distance % 100) >= *dial_state && *dial_state != 0{
            *zero_count += 1;
        }
        *dial_state = 100 + *dial_state + (turn_distance * turn_direction % 100);
    }
    *dial_state = *dial_state%100;
}

const INPUT_ERROR: &'static str = "Error parsing input file";
fn compute_line(line: String, dial_state: &mut i32, zero_count: &mut i32) {
    let turn_direction: i32 = parse_turn_direction(line.chars().nth(0).expect(INPUT_ERROR)).expect(INPUT_ERROR);
    let turn_distance: u32 = line[1..].parse::<u32>().expect(INPUT_ERROR);

    turn_dial(dial_state, zero_count, turn_distance as i32, turn_direction);
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

    let mut zero_count: i32 = 0;
    let mut dial_state: i32 = 50;
    match get_lines(&args[1]) {
        Ok(lines) => {
            for line in lines.map_while(Result::ok) {
                compute_line(line, &mut dial_state, &mut zero_count); 
            }
        }
        Err(e) => {
            return Err(format!("Unable to read file: {}", e.to_string()))?;
        }
    }

    println!("Amount of times dial landed on 0: {}", zero_count);

    Ok(())
}
