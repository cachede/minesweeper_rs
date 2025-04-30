use std::char::from_digit;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::io;
use std::io::ErrorKind;

//TODO: Maybe add extra parameters, if the user wants to provide an extra directory for input/output.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    if env::args().count() != 2 {
        println!("Please enter exactly one argument: The argument should point to a ms file");
        return Err(Box::from("Must enter exactly one ms file"));
    }

    let full_path = env::args().nth(1).unwrap();

    let minesweeper_string = fs::read_to_string(full_path.replace("\r", ""))?;

    let minesweeper_vec: Vec<Vec<char>> = minesweeper_string
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    check_valid_vec(&minesweeper_vec)?;

    let solved = solve_minesweeper(&minesweeper_vec)?;

    let result_string = convert_vec_to_string(&solved);

    let mut output_path = PathBuf::new();
    output_path.push(full_path);
    output_path.set_extension("out");
    fs::write(output_path, result_string)?;

    println!("Solved Minesweeper Map:");

    Ok(())

}

fn convert_vec_to_string(minesweeper_vec: &Vec<Vec<char>>) -> String {
    let mut result = String::new();

    for row in minesweeper_vec.iter() {
        for c in row {
            result.push(*c);
        }
        result.push('\n');
    }

    result
}

fn check_valid_vec(minesweeper_vec: &Vec<Vec<char>>) -> Result<(), io::Error> {

    let first_row = minesweeper_vec.first();

    let line_size = match first_row {
        Some(row) => row.len(),
        None => 0,
    };

    for row in minesweeper_vec.iter() {

        if row.len() != line_size {
            return Err(io::Error::new(ErrorKind::InvalidData, "Invalid Minesweeper Map"));
        }
    }

    Ok(())
}

/*
fn solve_minesweeper_rust(minesweeper_vec: &Vec<Vec<char>>) -> Result<Vec<Vec<char>>, io::Error> {

    /*
    let mut result: Vec<Vec<char>> = minesweeper_vec.clone();

    let mut iterator = minesweeper_vec.iter();

    let mut prev_line : Vec<char> = Vec::new();
    let mut cur_line : Vec<char>;
    let mut next_line : Vec<char> = iterator.next().unwrap().to_vec();

    for row in iterator {

        (prev_line, cur_line, next_line) = (cur_line, next_line, *row);


    }


     */

    Ok(result)
}

 */

pub fn solve_minesweeper(minesweeper_vec: &Vec<Vec<char>>) -> Result<Vec<Vec<char>>, io::Error> {
    let mut result: Vec<Vec<char>> = minesweeper_vec.clone();

    for (i, row) in minesweeper_vec.iter().enumerate() {

        for j in 0..row.len() {
            if *minesweeper_vec.get(i).unwrap().get(j).unwrap() == '*' {
                result[i][j] = '*';
                continue;
            }
            if *minesweeper_vec.get(i).unwrap().get(j).unwrap() != ' ' &&
                *minesweeper_vec.get(i).unwrap().get(j).unwrap() != '*' {

                return Err(io::Error::new(ErrorKind::InvalidData, "Invalid character"));
            }

            let mut bombs: u32 = 0;

            bombs += check_horizontal(&minesweeper_vec, i, j);
            bombs += check_vertical(&minesweeper_vec, i, j);
            bombs += check_diagonal(&minesweeper_vec, i, j);

            if bombs == 0 {
                result[i][j] = ' ';
            } else {
                result[i][j] = from_digit(bombs, 10).unwrap();
            }
        }
    }

    Ok(result)
}

fn check_horizontal(minesweeper_vec: &Vec<Vec<char>>, row: usize, column: usize) -> u32 {
    let mut bombs: u32 = 0;

    let column_left_search_index: i32 = column as i32 - 1;
    let column_right_search_index: i32 = column as i32 + 1;

    if let Some(char) = minesweeper_vec.get(row).unwrap().get(column_left_search_index as usize) {

        if *char == '*' {
            bombs += 1;
        }
    }

    if let Some(char) = minesweeper_vec.get(row).unwrap().get(column_right_search_index as usize) {

        if *char == '*' {
            bombs += 1;
        }
    }

    bombs
}

fn check_vertical(minesweeper_vec: &Vec<Vec<char>>, row: usize, column: usize) -> u32 {
    let mut bombs: u32 = 0;

    let row_up_search_index: i32 = row as i32 - 1;
    let row_down_search_index: i32 = row as i32 + 1;

    if let Some(char) = minesweeper_vec.get(row_up_search_index as usize) {
        if let Some(char) = minesweeper_vec.get(row_up_search_index as usize).unwrap().get(column) {
            if *char == '*' {
                bombs += 1;
            }
        }
    }

    if let Some(char) = minesweeper_vec.get(row_down_search_index as usize) {
        if let Some(char) = minesweeper_vec.get(row_down_search_index as usize).unwrap().get(column) {
            if *char == '*' {
                bombs += 1;
            }
        }
    }

    bombs
}

fn check_diagonal(minesweeper_vec: &Vec<Vec<char>>, row: usize, column: usize) -> u32 {
    let mut bombs: u32 = 0;

    let row_up_search_index: i32 = row as i32 - 1;
    let row_down_search_index: i32 = row as i32 + 1;
    let column_left_search_index: i32 = column as i32 - 1;
    let column_right_search_index: i32 = column as i32 + 1;

    if let Some(char) = minesweeper_vec.get(row_up_search_index as usize) {
        if let Some(char) = minesweeper_vec.get(row_up_search_index as usize).unwrap().get(column_left_search_index as usize) {
            if *char == '*' {
                bombs += 1;
            }
        }
    }

    if let Some(char) = minesweeper_vec.get(row_up_search_index as usize) {
        if let Some(char) = minesweeper_vec.get(row_up_search_index as usize).unwrap().get(column_right_search_index as usize) {
            if *char == '*' {
                bombs += 1;
            }
        }
    }

    if let Some(char) = minesweeper_vec.get(row_down_search_index as usize) {
        if let Some(char) = minesweeper_vec.get(row_down_search_index as usize).unwrap().get(column_left_search_index as usize) {
            if *char == '*' {
                bombs += 1;
            }
        }
    }

    if let Some(char) = minesweeper_vec.get(row_down_search_index as usize) {
        if let Some(char) = minesweeper_vec.get(row_down_search_index as usize).unwrap().get(column_right_search_index as usize) {
            if *char == '*' {
                bombs += 1
            }
        }
    }

    bombs
}

#[cfg(test)]
mod minesweeper_unit_tests {

    use all_asserts::*;
    use super::*;

    #[test]
    fn test_check_valid_vec() {

        let test_vec : Vec<Vec<char>> = vec![vec!['.', '.', '.'], vec!['.', '.', '.'], vec!['.', '.', '.'], vec!['.', '.', ',']];

        assert!(check_valid_vec(&test_vec));
    }

    #[test]
    fn test_check_valid_vec2() {

        let test_vec : Vec<Vec<char>> = vec![vec!['.'], vec!['.']];

        assert!(check_valid_vec(&test_vec));
    }

    #[test]
    fn test_check_valid_vec3() {

        let test_vec : Vec<Vec<char>> = vec![vec!['.']];

        assert!(check_valid_vec(&test_vec));
    }

    #[test]
    fn test_check_invalid_vec() {

        let test_vec : Vec<Vec<char>> = vec![vec!['.', '.'], vec!['.'], vec!['.']];

        assert_false!(check_valid_vec(&test_vec));
    }

    #[test]
    fn test_check_invalid_vec2() {

        let test_vec : Vec<Vec<char>> = vec![vec!['.'], vec!['.', '.'], vec!['.']];

        assert_false!(check_valid_vec(&test_vec));
    }

    #[test]
    fn test_check_invalid_vec3() {

        let test_vec : Vec<Vec<char>> = vec![
            vec!['.'],
            vec!['.', '.'],
            vec!['.', '.', '.']];

        assert_false!(check_valid_vec(&test_vec));
    }

    #[test]
    fn test_check_invalid_vec4() {

        let test_vec : Vec<Vec<char>> = vec![
            vec!['.'],
            vec!['.'],
            vec!['.', '.']];

        assert_false!(check_valid_vec(&test_vec));
    }

    #[test]
    fn test_solve_minesweeper() {

        let no_bomb_vec : Vec<Vec<char>> = vec![vec!['0', '0', '0'],
                                                vec!['0', '0', '0'],
                                                vec!['0', '0', '0']];

        let test_vec : Vec<Vec<char>> = vec![vec!['.', '.', '.'],
                                             vec!['.', '.', '.'],
                                             vec!['.', '.', '.']];

        let result_vec = solve_minesweeper(&test_vec);

        assert_eq!(no_bomb_vec, result_vec);
    }

    #[test]
    fn test_solve_minesweeper2() {

        let middle_bomb_vec : Vec<Vec<char>> = vec![vec!['1', '1', '1'],
                                                    vec!['1', '*', '1'],
                                                    vec!['1', '1', '1']];

        let test_vec : Vec<Vec<char>> = vec![vec!['.', '.', '.'],
                                             vec!['.', '*', '.'],
                                             vec!['.', '.', '.']];

        let result_vec = solve_minesweeper(&test_vec);

        assert_eq!(middle_bomb_vec, result_vec);
    }

    #[test]
    fn test_solve_minesweeper3() {

        let one_line_vec : Vec<Vec<char>> = vec![vec!['1', '*', '2', '*']];

        let test_vec : Vec<Vec<char>> = vec![vec!['.', '*', '.', '*']];

        let result_vec = solve_minesweeper(&test_vec);

        assert_eq!(one_line_vec, result_vec);
    }

    #[test]
    fn test_solve_minesweeper4() {

        let max_bombs_vec = vec![vec!['*', '*', '*'],
                                                vec!['*', '8', '*'],
                                                vec!['*', '*', '*']];

        let test_vec : Vec<Vec<char>> = vec![vec!['*', '*', '*'],
                                            vec!['*', '.', '*'],
                                            vec!['*', '*', '*']];

        let result_vec = solve_minesweeper(&test_vec);

        assert_eq!(max_bombs_vec, result_vec);
    }

    #[test]
    fn test_solve_minesweeper5() {

        let output = vec![
            vec!['*', '1', '1', '*'],
            vec!['2', '2', '2', '1'],
            vec!['2', '*', '2', '1'],
            vec!['*', '2', '2', '*']
        ];

        let input = vec![
            vec!['*', '.', '.', '*'],
            vec!['.', '.', '.', '.'],
            vec!['.', '*', '.', '.'],
            vec!['*', '.', '.', '*'],
        ];

        let result = solve_minesweeper(&input);

        assert_eq!(output, result);

    }

    #[test]
    fn test_solve_minesweeper6() {

        /*
        let absolute_path = env::current_dir().unwrap().display().to_string() + "\\";
        let relative_path: String = env::args().skip(1).collect();
        let full_path: String = absolute_path + &relative_path;

        let full_path = "C:\\Users\\danie\\OneDrive\\Desktop\\Rust\\Praktikum\\Aufgabe1\\src\\MinesweeperFiles\\ms1.mines";

        let minesweeper_string = file_to_string(&full_path);

        match minesweeper_string {
            Some(minesweeper_string) => {

                let minesweeper_vec: Vec<Vec<char>> = minesweeper_string
                    .lines()
                    .map(|line| line.chars().collect())
                    .collect();

                if !check_valid_vec(&minesweeper_vec) {
                    eprintln!("Invalid map: it should be a rectangle");
                    return;
                }

                let solved = solve_minesweeper(&minesweeper_vec);

                let result_string = convert_vec_to_string(&solved);


                fs::write("C:\\Users\\danie\\OneDrive\\Desktop\\Rust\\Praktikum\\Aufgabe1\\src\\Output\\test_output.txt", result_string).expect("Unable to write file");


            },
            None => {
                println!("No file found at {}", &full_path);
            }
        }

    */


    }

}
