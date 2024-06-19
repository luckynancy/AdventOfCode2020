use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn day_two() -> io::Result<()> {
    // Open the file
    let file = File::open("./data/day2.txt")?;

    // Create a buffered reader
    let reader = BufReader::new(file);

    // Create counting variables
    let (mut count1, mut count2) = (0, 0) ;

    // Iterate over each line in the file
    for line in reader.lines() {

        // Each line is a Result<String, io::Error>
        let line = line?; // Handle the Result, propagating errors
        
        // Split the string by multiple delimiters
        let parts : Vec<&str> = line.split(|c| c == ' ' || c == '-' || c == ':').filter(|part| !part.is_empty()).collect();
        
        // Convert the limits to i32 
        let num1: usize = parts[0].parse().unwrap();
        let num2: usize = parts[1].parse().unwrap();

        // Convert the target character
        let target: char = parts[2].chars().next().unwrap();

        // Collect the positions of the character in the string
        let positions: Vec<usize> = parts[3].char_indices().filter_map(|(i, c)| if c == target { Some(i) } else { None }).collect();

        // Count the occurrence
        let occurrence: usize = positions.len().try_into().unwrap();

        // Part I, find the occurrence of the specific character. 
        match occurrence{
            x if  x <= num2 && x >= num1 => count1 += 1,
            _ => ()
        } 
       
        // Adjust the no "index zero" policy in Part II
        let num1 = num1-1;
        let num2 = num2-1;

        // Part II, find if the positions are as per specified
        if (positions.contains(&num1) && !positions.contains(&num2)) || (!positions.contains(&num1) && positions.contains(&num2)) {
            count2 += 1
        }
    }
    // Return the vector of numbers
    println! ("Part I: There are {} valid passwords", count1);
    println! ("Part II: There are {} valid passwords", count2);

    Ok(())
}