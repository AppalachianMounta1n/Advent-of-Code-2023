use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // Get the file path from the command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];

    // Read the file and process its contents
    if let Ok(file) = File::open(&file_path) {
        let mut total_sum = 0;

        for line in io::BufReader::new(file).lines() {
            if let Ok(line) = line {
                let first: u32 = line.chars().find_map(|c| c.to_digit(10)).unwrap_or_else(|| panic!("No numbers in line: {}", line)); //first number in the line
                let last: u32 = line.chars().rev().find_map(|c| c.to_digit(10)).unwrap_or_else(|| panic!("No numbers in line: {}", line)); //last number in the line

                total_sum += first * 10 + last;
            }
        }

        // Output the total sum of all numbers in the file
        println!("Total Sum: {}", total_sum);
    } else {
        eprintln!("Error opening the file: {}", file_path);
    }

    Ok(())
}