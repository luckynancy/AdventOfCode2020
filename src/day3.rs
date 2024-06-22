use std::io;
use std::collections::HashSet;
use crate::utils::read_lines_from_file;


pub fn day_three() -> io::Result<()> {

    let path = "./data/day3.txt";

    match read_lines_from_file(path) {
        Ok(lines) => {

            // Define a HashSet to store pairs
            let mut pairs: HashSet<(usize, usize)> = HashSet::new();

            // Insert some pairs into the HashSet
            pairs.insert((1, 1));
            pairs.insert((3, 1));
            pairs.insert((5, 1));
            pairs.insert((7, 1));
            pairs.insert((1, 2));

            let mut part_2 = 1;

            // Iterate over the HashSet and print output
            for &(right, down) in &pairs {
                let count = find_tree( &lines, right, down);
                if right == 3 && down == 1{
                    println! ("Part I: There are {} trees", count);
                }
                part_2 *= count;
            }
            println! ("Part II: There product is {}", part_2);

        }
        Err(e) => {
            eprintln!("Error reading file: {:?}", e);
        }
    }
    
    Ok(())
}

fn find_tree(lines:&Vec<String>, right:usize, down: usize) -> usize{
    let mut count = 0; 
    let mut pos_x = 0;

    // Create a range from 0 to 323 (inclusive) with a step of 3
    let pos_y_step: Vec<_> = (down..lines.len()).step_by(down).collect();

    // Print the range to verify
    for pos_y in pos_y_step {
        pos_x += right; 
        let target_char = lines[pos_y].chars().cycle().nth(pos_x);
        if let Some('#') = target_char {
            count += 1;
        } else {
            count += 0;
        }
    }
    count
}