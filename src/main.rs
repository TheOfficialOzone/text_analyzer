



use std::{collections::HashMap, io::{Write, self}};
use std::fs;
fn main() -> Result<(), String> {
    //Gets the file argument
    let file_path = match std::env::args().nth(1) {
        Some(file_path) => file_path,
        None => return Err(String::from("No file path specified.")),
    };

    // Read the file
    let buf = match fs::read_to_string(file_path) {
        Ok(string) => string,
        Err(error) => return Err(error.to_string()),
    };

    //Makes the character map / vector
    let mut char_map = HashMap::new();
    let mut char_vec : Vec<(char, usize)> = Vec::new();

    // For the first 256 ascii chars, automatically add them
    for i in 0..256 {
        char_vec.push((char::from_u32(i).unwrap(), 0));
    }

    // Used to count the total number of characters
    let mut total_character_amount : usize = 0;

    // Loops through all the characters counting them all
    for char in buf.chars() {
        //Ignores whitespace
        if char.is_whitespace() { continue; }

        //If it's ascii immediately add it (Quicker than always accessing a hash-map)
        if char.is_ascii() {
            char_vec[char as usize].1 += 1;
            total_character_amount += 1;
            continue;
        }

        // If it's any other character (UNICODE)
        // We get it's position in the array by using a character map
        
        // Gets the array position of the character (DEFAULTs to the end of the array)
        let mut array_pos : usize = char_vec.len();
        match char_map.get(&char) {
            Some(x) => (array_pos = *x),
            None => {
                match char_map.insert(char, array_pos) {
                    Some(_x) => (return Err(String::from("Char map inserted the same error twice!"))),
                    None => char_vec.push((char, 0)),
                };
            },
        };
        
        //Adds to the total character count
        char_vec[array_pos].1 += 1;
        total_character_amount += 1;
    }

    // Sorts the Vector
    char_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // Buffers the prints
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);

    //Prints out each chars occurance ratio
    for val in char_vec {
        //Ignore any characters that never occured
        if val.1 == 0 { continue; }

        //Will print the percentages to 3 decimal places
        match writeln!(handle, "{} : {}\t{:.3}%", val.0, val.1, ((val.1 as f64 / total_character_amount as f64) * 100.0))  {
            Ok(_) => (),
            Err(error) => return Err(error.to_string()),
        };
    };
    
    //Adds the total amount of characters
    match writeln!(handle, "Total character amount {}", total_character_amount) {
        Err(error) => return Err(error.to_string()),
        _ => (),
    };
    
    //Print it all out to the terminal
    match handle.flush() {
        Err(error) => return Err(error.to_string()),
        _ => (),
    };
    
    Ok(())
}
