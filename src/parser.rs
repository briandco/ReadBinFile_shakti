
use std::{fs::File, io::Read, process};

pub fn read_file()-> (String,usize){

   
    let file_path = "/home/cbq2kor/Desktop/DevSpace/Test/Shakti/deployShakti/code.mem";
    

    // Open the file
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open file '{}': {}", file_path, e);
            process::exit(1);
        }
    };

     // Read the hexadecimal string from standard input
    let mut hex_string = String::new();
    let _ = file.read_to_string(&mut hex_string);

    let mut flash_data_vec: Vec<u8>  = Vec::new();
 
    for (_i, byte) in hex_string.split("\n").enumerate(){
        
        flash_data_vec.extend(hex_to_bytes(byte));
       
    }

    let content = format!("{:?}", flash_data_vec);
    (content, flash_data_vec.len())
    
}

fn hex_to_bytes(hex: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut iter = hex.chars().peekable();
 
    while let Some(c1) = iter.next() {
        if let Some(c2) = iter.next() {
            let byte_str = format!("{}{}", c1, c2);
            if let Ok(byte) = u8::from_str_radix(&byte_str, 16) {
                bytes.push(byte);
            } else {
                // Handle invalid hexadecimal characters
                eprintln!("Invalid hexadecimal string: {}", byte_str);
            }
        }
    }
 
    bytes
}