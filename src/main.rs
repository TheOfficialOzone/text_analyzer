


use std::{fs::File, collections::HashMap, io::{Read, Write, self}};

fn main() -> Result<(), String> {
    // Gets the arguments
    let mut args = std::env::args();

    //Gets the file argument
    let file_arg;
    match args.nth(1) {
        Some(arg) => file_arg = arg,
        None => return Err(String::from("No file path specified.")),
    }

    //Opens the file
    let mut file;
    match File::open(file_arg.clone()) {
        Ok(fil) => file = fil,
        Err(error) => return Err(format!("{} For: '{}'", error, file_arg)),
    };

    //Reads the file to a string
    let mut buf : String = String::new();
    match file.read_to_string(&mut buf) {
        Ok(_) => (),
        Err(error) => return Err(format!("{}", error)),
    }

    //Makes the character map / vector
    let mut char_map = HashMap::new();
    let mut char_vec : Vec<(char, usize)> = Vec::new(); //vec![(0; 256];

    // For the first 256 ascii chars, automatically add them
    for i in 0..256 {
        char_vec.push((char::from_u32(i).unwrap(), 0));
    }

    //Counts the total number of characters
    let mut total_character_amount : usize = 0;

    //Loops through all the characters
    for char in buf.chars() {
        //Ignores whitespace
        if char.is_whitespace() { continue; }

        //If it's ascii immediately add it
        if char.is_ascii() {
            char_vec[char as usize].1 += 1;
            total_character_amount += 1;
            continue;
        }

        // If it's any other character (UNICODE)
        // We get it's position in the array by using a character map
        
        //Gets the array position of the character (DEFAULTs to the end of the array)
        let mut array_pos : usize = char_vec.len();
        match char_map.get(&char) {
            Some(x) => (array_pos = *x),
            None => {
                match char_map.insert(char, array_pos) {
                    Some(_x) => (panic!("Char map inserted same char twice")),
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
        if val.1 == 0 { continue; }

        match writeln!(handle, "{} : {}\t{:.3}%", val.0, val.1, ((val.1 as f64 / total_character_amount as f64) * 100.0))  {
            Ok(_) => (),
            Err(error) => return Err(format!("{}", error)),
        };
    };
    
    //Adds the total amount of characters
    match writeln!(handle, "Total character amount {}", total_character_amount) {
        Ok(_) => (),
        Err(error) => return Err(format!("{}", error)),
    };
    
    //Print it all out to the terminal
    match handle.flush() {
        Ok(_) => (),
        Err(error) => println!("{}", error),
    };
    
    Ok(())
}
