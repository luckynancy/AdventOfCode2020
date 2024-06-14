use std::io;
use std::collections::HashSet;
use crate::utils::read_numbers_from_file;

pub fn day_one()-> io::Result<()>{
    // Specify the path to the file
    let path = "./data/day1.txt";
    
    // Specify the target sum
    let target_sum = 2020;

    match read_numbers_from_file(path) {
        Ok(numbers) => {
            // Part I
            match find_pair_with_sum(&numbers, target_sum) {
                Some((num1, num2)) => {
                    println!("Part I: The numbers are {num1} and {num2}, and their product is {}", num1*num2);
                }
                None => {
                    println!("No pair of numbers found that sums to {target_sum}");
                }
            }  
            
            // Part II
            for (i, &num1) in numbers.iter().enumerate() {
                let current_sum = target_sum - num1;
                match find_pair_with_sum(&numbers[i+1..], current_sum) {
                    Some((num2, num3)) => {
                        println!("Part II: The numbers are {num1}, {num2} and {num3}, and their product is {}", num1*num2*num3);
                    }
                    None => {
                    }
                }
            }

        }
        Err(e) => {
            eprintln!("Error reading file: {:?}", e);
        }
    }

    Ok(())
}


fn find_pair_with_sum(numbers: &[i32], target_sum: i32) -> Option<(i32, i32)> {

    let mut seen = HashSet::new();
    for &number in numbers {
        let complement = target_sum - number;
        if seen.contains(&complement) {
            return Some((number, complement));
        }
        seen.insert(number);
    }  
    None
    
}

