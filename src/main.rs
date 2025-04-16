use std::char::from_digit;
use std::env;
use std::fs;

fn main() {

    if env::args().count() != 2  {
        println!("Please enter exactly one argument: ");
        return;
    }

    let minesweeper_string = file_to_string();

    let minesweeper_vec : Vec<Vec<char>> = minesweeper_string.lines().map(|line| line.chars().collect()).collect();

    if !check_valid_vec(&minesweeper_vec) {
        eprintln!("Invalid map");
        return;
    }

    let solved = solve_minesweeper(&minesweeper_vec);

    let result_string = convert_vec_to_string(&solved);

    println!("Solved Minesweeper Map:");
    println!("{}", result_string);

}

fn convert_vec_to_string(minesweeper_vec : &Vec<Vec<char>>) -> String {

    let mut result = String::new();

    for row in minesweeper_vec.iter() {

        for c in row {
            result.push(*c);
        }
        result.push('\n');
    }

    result
}

fn file_to_string() -> String {

    let absolute_path = env::current_dir().unwrap().display().to_string() + "\\";
    let relative_path: String = env::args().skip(1).collect();
    let full_path: String = absolute_path + &relative_path;

    match fs::read_to_string(&full_path) {
        Ok(content) => content.trim().replace("\r", "").to_string(),
        Err(e) => {
            println!("Error reading file {}", e.to_string());
            std::process::exit(1);
        }
    }
}

fn check_valid_vec(minesweeper_vec : &Vec<Vec<char>>) -> bool {

    let first_row_len = minesweeper_vec[0].len();

    for i in minesweeper_vec.iter() {

        if i.len() != first_row_len {
            return false;
        }
    }

    true
}

fn solve_minesweeper(minesweeper_vec : &Vec<Vec<char>>) -> Vec<Vec<char>> {

    let mut result: Vec<Vec<char>> = minesweeper_vec.clone();

    for (i, row) in result.iter_mut().enumerate() {

        for (j, c) in row.iter_mut().enumerate() {

            if minesweeper_vec[i][j] == '*' {
                result[j][i] = '*';
                continue;
            }

            let mut bombs : u32 = 0;

            // check horizontally
            // check vertically
            // check diagonal

            result[j][i] = from_digit(bombs, 10).unwrap();

        }
    }

    result
}

/*
fn solve_minesweeper(minesweeper_map : &String) -> String {

    let mut result_map : String = String::new();
    let minesweeper_map_width = minesweeper_map.split('\n').next().unwrap().len();
    println!("Minesweeper Map Width: {}", minesweeper_map_width);
    let minesweeper_map_height = minesweeper_map.split('\n').count();
    println!("Minesweeper Map Height: {}", minesweeper_map_height);

    for (index, line) in minesweeper_map.split('\n').enumerate() {

        for (i, c) in line.chars().enumerate() {

            if c != '.' && c != '\n'{
                result_map.push('*');
                continue;
            }

            let mut bombs : u32 = 0;

            bombs += check_map_horizontal(&minesweeper_map, index, i, minesweeper_map_width);
            bombs += check_map_vertical(&minesweeper_map, index, i, minesweeper_map_width, minesweeper_map_height);
            bombs += check_map_diagonal(&minesweeper_map, index, i, minesweeper_map_width, minesweeper_map_height);

            result_map.push(from_digit(bombs, 10).unwrap());
        }

        result_map.push('\n');
    }


    result_map
}

fn check_map_horizontal(minesweeper_map : &String, line_index : usize, column_index : usize, map_length : usize) -> u32 {

    let local_map = minesweeper_map.split('\n').skip(line_index).next().unwrap().to_string();

    if column_index == 0 {
        // checke nur rechte seite

        if local_map.chars().nth(column_index + 1).unwrap() == '*' {
            return 1;
        } else {
            return 0;
        }

    }
    if column_index == map_length - 1 {
        // checke nur linke seite

        if local_map.chars().nth(column_index - 1).unwrap() == '*' {
            return 1;
        } else {
            return 0;
        }

    }

    let mut bombs : u32 = 0;
    let left_side = column_index - 1;
    if local_map.chars().nth(left_side).unwrap() == '*' {
        bombs += 1;
    }
    let right_side = column_index + 1;
    if local_map.chars().nth(right_side).unwrap() == '*' {
        bombs += 1;
    }


    bombs
}

fn check_map_vertical(minesweeper_map : &String, line_index : usize, column_index : usize, map_length : usize, map_width : usize) -> u32 {

    let local_map : String = minesweeper_map.split('\n').collect();

    //nur unten
    if line_index == 0 {

        let one_down = column_index + map_length;
        if local_map.chars().nth(column_index + map_length).unwrap() == '*' {
            return 1;
        } else {
            return 0;
        }
    }

    //nur oben
    if line_index == map_width - 1 {

        if local_map.chars().nth(line_index * map_length + column_index - map_length).unwrap() == '*' {
            return 1;
        } else {
            return 0;
        }
    }

    let mut bombs : u32 = 0;
    let top = line_index * map_length + column_index - map_length;
    if local_map.chars().nth(top).unwrap() == '*' {
        bombs += 1;
    }
    let bottom = line_index * map_length + column_index + map_length;
    if local_map.chars().nth(bottom).unwrap() == '*' {
        bombs += 1;
    }

    bombs
}

fn check_map_diagonal(minesweeper_map : &String, line_index : usize, column_index : usize, map_length : usize, map_width : usize) -> u32 {

    let local_map : String = minesweeper_map.split('\n').collect();
    let mut bombs : u32 = 0;




    bombs
}

*/

fn check_valid_map(minesweeper_map : &String) -> bool {

    let mut whatever = minesweeper_map.split('\n');
    let first_row = whatever.next().unwrap().len();

    for str in whatever {
        if str.len() != first_row {
            return false;
        }
    }

    true
}
