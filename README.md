# minesweeper_rs
A minesweeper solution generator build in rust. 
Takes an input minesweeper file, solves it and produces an output file.

#### Project structure
.
├── src                     # Source files
│   ├── system_tests        # Test directory
│        ├── test_files     # Example test input files
│        ├── run_tests.sh   # Test script
│   ├── main.rs             # Application
└── README.md

#### Usage
./minesweeper [input-file]

#### Input file
The input file marks free fields as a whitespace and bombs are marked by '*'.
The minesweeper input file should be a valid rectangle (every line has the same size). Otherwise no output file is generated.

#### Systemtests
In folder system_tests is a shell script, which tests the program with a set of example test input files.

