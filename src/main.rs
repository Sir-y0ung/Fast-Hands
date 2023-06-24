use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use rand::Rng;
use std::{i64, u8};
use std::num::ParseIntError;
use std::str::FromStr;
use std::env;
use std::process;

fn read_file_to_bytes(filename: &str) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(filename)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}

fn write_bytes_to_file(bytes: &[u8], filename: String) -> std::io::Result<()> {
    let mut file = File::create(filename)?;

    file.write_all(bytes)?;

    Ok(())
}

fn bitwise(a: u8, b: u8) -> u8 {
    (((a | b) & !(a & b)) & 0xff)
}



fn stage1(byte_array: Vec<u8>, rand_value: u8) -> Vec<u8> {
    let mut bytes = byte_array.to_vec();
    let size = bytes.len();
    let mut result: Vec<u8> = Vec::with_capacity(size);

    for byte in bytes.iter() {
        result.push((bitwise(byte, rand_value)) & 0xff)
    }

    result
}

fn convert_to_hex(input_string: String) -> Option<u64> {
    let hex_string = format!("0x{}", input_string);

    match i64::from_str_radix(&hex_string, 16) {
        Ok(number) => Some(number as u64),
        Err(_) => None,
    }
}

fn stage2(byte_array: Vec<u8>) -> Vec<u8>{
    let mut bytes = byte_array.to_vec();
    let mut size = bytes.len();
    let mut result: Vec<u8> = Vec::new();

    for number in 0..=size/3 {

        let mut compination: String = format!("{:02x}{:02x}{:02x}", bytes[number], bytes[number + 1], bytes[number + 2]);

        compination = shuffle(&compination);

        let  parsed_int = u8::from_str_radix(&compination[0..2], 16);
        match parsed_int {
            Ok(value) => result.push(value),
            Err(err) => println!("Failed to parse: {}", err),
        }

        let parsed_int = u8::from_str_radix(&compination[2..4], 16);
        match parsed_int {
            Ok(value) => result.push(value),
            Err(err) => println!("Failed to parse: {}", err),
        }

        let parsed_int = u8::from_str_radix(&compination[4..6], 16);
        match parsed_int {
            Ok(value) => result.push(value),
            Err(err) => println!("Failed to parse: {}", err),
        }
    }

    result
}

fn shuffle(input: &str) -> String {
    let first_part = &input[4..6];
    let second_part = &input[0..4];

    let result = format!("{}{}", first_part, second_part);
    result
}

fn encrypt(bytes: Vec<u8>) -> Vec<u8> {
    let mut data = bytes.to_vec();
    let mut rng = rand::thread_rng();
    let random_number: u8 = rng.gen_range(10..=99);

    data = stage1(data, random_number);
    data = stage2(data);
    data
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(first_arg) = args.get(1) {

        let filename = first_arg;
        let encrypted_file = filename.to_owned() + ".enc";
        let mut raw_data: Vec<u8> = Vec::new();

        match read_file_to_bytes(filename) {
            Ok(bytes) => {
                println!("Trying to encrypt file");
                raw_data = bytes;
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        }

        if raw_data.len() % 3 != 0 {
            println!("Invalid file size!");
            process::exit(1);
        }

        let enc_data = encrypt(raw_data);

        if let Err(err) = write_bytes_to_file(&enc_data, encrypted_file) {
            println!("Error: {}", err);
        } else {
            println!("File encrypted successfully.");
        }
    }
    else {
        println!("usage: ./program <filename>");
    }

}
