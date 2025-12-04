use std::error::Error;
use std::fs::File;
use std::env;
use std::io::{self, BufRead};
use std::path::Path;

fn pos_is_occupied(grid: &Vec<Vec<char>>, x: i32, y: i32) -> bool {
    // Make sure we don't go out of bounds in the grid
    if y >= grid.len() as i32 || y < 0{
        return false;
    }

    // out-of-bounds check for x is handled by unwrap_or
    return *grid[y as usize].get(x as usize).unwrap_or(&'.') == '@';
}

fn solve(grid: &mut Vec<Vec<char>>, result: &mut u32) -> bool {
    let grid_len = grid.len() as i32;
    let row_len = grid[0].len() as i32;

    let mut changed_grid: bool = false;
    // Loop over every grid position
    for y in 0..grid_len {
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

                if surrounding_occupations < 4 {
                    grid[y as usize][x as usize] = '.';
                    changed_grid = true;
                    *result += 1;
                }
            }
        }
    }
    changed_grid
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            print!("{}", grid[i][j]);
        }
        println!("");
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
    let mut grid: Vec<Vec<char>> = vec![];
    let mut str_len: usize = 0;
    match get_lines(&args[1]) {
        Ok(lines) => {
            for line in lines.map_while(Result::ok) {
                if str_len == 0 {
                    str_len = line.len();
                } else if str_len != line.len() {
                    return Err(format!("Invalid input: incorrect line length for line {}", line))?;
                }
                grid.push(line.chars().collect());
            }
        }
        Err(e) => {
            return Err(format!("Unable to read file: {}", e.to_string()))?;
        }
    }

    let mut result: u32 = 0;

    loop {
        if solve(&mut grid, &mut result) == false {
            break;
        }
    }
    // print_grid(&grid);
    println!("Result: {}", result);

    Ok(())
}
