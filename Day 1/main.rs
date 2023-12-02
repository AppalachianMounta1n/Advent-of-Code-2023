use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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
                let numbers: Vec<i32> = line
                    .chars()
                    .filter(|c| c.is_numeric())
                    .flat_map(|c| c.to_string().parse())
                    .collect();

                // If there are no numbers, continue to the next line
                if numbers.is_empty() {
                    continue;
                }

                // If there's only one number, repeat it in the array
                let repeated_numbers: Vec<i32> = if numbers.len() == 1 {
                    vec![numbers[0]; line.matches(char::is_numeric).count()]
                } else {
                    numbers
                };

                // Output the array of numbers for each line
                println!("Numbers in line: {:?}", repeated_numbers);

                // Calculate and accumulate the sum for each line
                let line_sum: i32 = repeated_numbers.iter().sum();
                total_sum += line_sum;
            }
        }

        // Output the total sum of all numbers in the file
        println!("Total Sum: {}", total_sum);
    } else {
        eprintln!("Error opening the file: {}", file_path);
    }

    Ok(())
}