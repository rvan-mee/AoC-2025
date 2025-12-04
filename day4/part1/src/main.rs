use std::error::Error;
use std::fs::File;
use std::env;
use std::io::{self, BufRead};
use std::path::Path;

fn pos_is_occupied(grid: &Vec<String>, x: i32, y: i32) -> bool {
    // Make sure we don't go out of bounds in the grid
    if y >= grid.len() as i32 || y < 0{
        return false;
    }

    // out-of-bounds check for the string is handled by unwrap_or
    return grid[y as usize].chars().nth(x as usize).unwrap_or('.') == '@';
}

fn solve(grid: &Vec<String>, result: &mut u32) -> Vec<String> {
    let grid_len = grid.len() as i32;
    let row_len = grid[0].len() as i32;

    let mut solved_grid: Vec<String> = vec![];

    // Loop over every grid position
    for y in 0..grid_len {
        // Add new string to solved grid
        solved_grid.push("".to_string());
        for x in 0..row_len {

            // Check if it is occupied, if it is check if it has >=4 surrounding occupations
            let mut surrounding_occupations = 0;
            if pos_is_occupied(&grid, x, y) {
                for y_check in y-1..=y+1 {
                    for x_check in x-1..=x+1 {
                        if !(y_check == y && x_check == x) && pos_is_occupied(&grid, x_check, y_check) {
                            surrounding_occupations += 1;
                        }
                    }
                }
            }

            if pos_is_occupied(&grid, x, y) && surrounding_occupations < 4 {
                solved_grid[y as usize].push('x');
                *result += 1;
            } else {
                solved_grid[y as usize].push(grid[y as usize].chars().nth(x as usize).unwrap_or('.'));
            }
        }
    }
    solved_grid
}

fn print_grid(grid: &Vec<String>) {
    for i in 0..grid.len() {
        println!("{}", grid[i]);
    } 
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

    // Handle input
    let mut grid: Vec<String> = vec![];
    let mut str_len: usize = 0;
    match get_lines(&args[1]) {
        Ok(lines) => {
            for line in lines.map_while(Result::ok) {
                if str_len == 0 {
                    str_len = line.len();
                } else if str_len != line.len() {
                    return Err(format!("Invalid input: incorrect line length for line {}", line))?;
                }
                grid.push(line);
            }
        }
        Err(e) => {
            return Err(format!("Unable to read file: {}", e.to_string()))?;
        }
    }

    let mut result: u32 = 0;

    // let solved_grid: Vec<String> = solve(&grid);
    // print_grid(&solved_grid);
    solve(&grid, &mut result);
    println!("Result: {}", result);

    Ok(())
}
