// rust cli tool to read files and print their hex values
use colored::Colorize;
use std::env;
use hex::FromHex;
use std::fs::File;
use std::io::{BufReader, Read};

extern crate colored;

fn main() {
    println!("{}","\t\tHex Editor".bold().bright_green());
    println!("{}","\t\tby editor99".bold().bright_green());
    println!("{}","\t\t\nDrag & Drop a file to open it.".blue());
    println!("{}","\t\t\nAlternatively, paste the file location here:.".blue());
    // get the file path from user input
    let mut file_path = String::new();
    std::io::stdin().read_line(&mut file_path).expect("Error reading file path");
    file_path = file_path.trim().to_string();
    let file = File::open(file_path).expect("File not found");
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).expect("Error reading file");
    let hex_string = hex::encode(buffer);
    let bytes = hex::decode(&hex_string).unwrap();

    // print hex values in a grid
    println!("{}","Hex View".bold().bright_green());
    let mut row_count = 0;
    let mut row = Vec::new();
    for (i, byte) in bytes.iter().enumerate() {
        row.push(format!("{:02X}", byte));
        if (i + 1) % 16 == 0 {
            print!("{}: {}\n", format!("{:08X}", row_count * 16).bright_yellow(), row.join(" "));
            row_count += 1;
            row.clear();
        } else if (i + 1) % 8 == 0 {
            row.push(" ".to_string());
        }
    }
    if !row.is_empty() {
        while row.len() < 16 {
            row.push("  ".to_string());
        }
        print!("{}: {}\n", format!("{:08X}", row_count * 16).bright_yellow(), row.join(" "));

    }

    // upon done, print the file size, and ask user if they want to exit
    println!("File size: {} bytes", bytes.len().to_string().bright_yellow());

    // write to file 
    

    println!("{}","Press Enter to exit.".bold().bright_green());
    let mut exit = String::new();
}
