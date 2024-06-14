use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn read_numbers_from_file(path: &str) -> io::Result<Vec<i32>> {
    // Open the file
    let file = File::open(path)?;
    
    // Create a buffered reader
    let reader = BufReader::new(file);
    
    // Initialize a vector to hold the numbers
    let mut numbers = Vec::new();
    
    // Iterate over each line in the file
    for line in reader.lines() {
        // Each line is a Result<String, io::Error>
        let line = line?;
        
        // Try to parse the line as an i32
        match line.trim().parse::<i32>() {
            Ok(number) => numbers.push(number),
            Err(e) => eprintln!("Failed to parse line '{line}': {e}"),
        }
    }
    
    // Return the vector of numbers
    Ok(numbers)
}