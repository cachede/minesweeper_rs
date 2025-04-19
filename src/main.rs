use std::char::from_digit;
use std::env;
use std::fs;

fn main() {

    if env::args().count() != 2  {
        println!("Please enter exactly one argument: The argument should point to a ms file");
        return;
    }

    let minesweeper_string = file_to_string();

    let minesweeper_vec : Vec<Vec<char>> = minesweeper_string.lines().map(|line| line.chars().collect()).collect();

    if !check_valid_vec(&minesweeper_vec) {
        eprintln!("Invalid map: it should be a rectangle");
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


    for (i, row) in minesweeper_vec.iter().enumerate() {

        for j in 0..row.len() {

            if minesweeper_vec[i][j] == '*' {
                result[i][j] = '*';
                continue;
            }

            let mut bombs : u32 = 0;

            bombs += check_horizontal(&minesweeper_vec, i, j);
            bombs += check_vertical(&minesweeper_vec, i, j);
            bombs += check_diagonal(&minesweeper_vec, i, j);

            result[i][j] = from_digit(bombs, 10).unwrap();

        }
    }

    result
}

fn check_horizontal(minesweeper_vec : &Vec<Vec<char>>, row : usize, column : usize) -> u32 {

    let mut bombs : u32 = 0;

    let column_left_search_index : i32 = column as i32 - 1;
    let column_right_search_index : i32 = column as i32 + 1;

    if column_left_search_index >= 0 {

        if minesweeper_vec[row][column_left_search_index as usize] == '*' {

            bombs += 1;
        }
    }
    if column_right_search_index < minesweeper_vec[row].len() as i32 {

        if minesweeper_vec[row][column_right_search_index as usize] == '*' {

            bombs += 1;
        }
    }

    bombs
}

fn check_vertical(minesweeper_vec : &Vec<Vec<char>>, row : usize, column : usize) -> u32 {

    let mut bombs : u32 = 0;

    let row_up_search_index : i32 = row as i32 - 1;
    let row_down_search_index : i32 = row as i32 + 1;

    if row_up_search_index >= 0 {

        if minesweeper_vec[row_up_search_index as usize][column] == '*' {
            bombs += 1
        }
    }
    if row_down_search_index < minesweeper_vec.len() as i32 {

        if minesweeper_vec[row_down_search_index as usize][column] == '*' {
            bombs += 1;
        }
    }

    bombs
}

fn check_diagonal(minesweeper_vec: &Vec<Vec<char>>, row : usize, column : usize) -> u32 {

    let mut bombs : u32 = 0;

    let row_up_search_index : i32 = row as i32 - 1;
    let row_down_search_index : i32 = row as i32 + 1;
    let column_left_search_index : i32 = column as i32 - 1;
    let column_right_search_index : i32 = column as i32 + 1;

    if row_up_search_index >= 0 && column_left_search_index >= 0 {

        if minesweeper_vec[row_up_search_index as usize][column_left_search_index as usize] == '*' {
            bombs += 1;
        }
    }
    if row_up_search_index >= 0 && column_right_search_index < minesweeper_vec[row_up_search_index as usize].len() as i32 {

        if minesweeper_vec[row_up_search_index as usize][column_right_search_index as usize] == '*' {
            bombs += 1;
        }
    }
    if row_down_search_index < minesweeper_vec.len() as i32 && column_left_search_index >= 0 {

        if minesweeper_vec[row_down_search_index as usize][column_left_search_index as usize] == '*' {
            bombs += 1;
        }
    }
    if row_down_search_index < minesweeper_vec.len() as i32 && column_right_search_index < minesweeper_vec[row_down_search_index as usize].len() as i32 {

        if minesweeper_vec[row_down_search_index as usize][column_right_search_index as usize] == '*' {
            bombs += 1;
        }
    }

    bombs
}

