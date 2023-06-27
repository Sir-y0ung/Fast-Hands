use std::fs::File;
use rand::prelude::*;
use std::io::{Read, Write};
use rand::seq::SliceRandom;
use rand::{Rng, thread_rng};
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

fn level1(a: u8, b: u8) -> u8 {
    (((a | b) & !(a & b)) & 0xff)
}

fn stage1(byte_array: Vec<u8>, rand_value: u8) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();

    for byte in byte_array.iter() {
        result.push((level1(*byte, rand_value)) & 0xff)
    }

    result
}

fn stage2(byte_array: &[u8], shuffle_list: &[usize]) -> Vec<u8> {
    let mut result: Vec<u8> = vec![0; shuffle_list.len()];

    for i in 0..byte_array.len() {
        result[shuffle_list[i]] = byte_array[i];
    }

    result
}

fn generate_unique_list(n: usize, seed: u16) -> Vec<usize> {
    let mut unique_list: Vec<usize> = (0..=n).collect();
    let mut rng = StdRng::seed_from_u64(seed as u64);
    unique_list.shuffle(&mut rng);
    unique_list
}

fn shuffle(input: &str) -> String {
    let second_part = &input[4..6];
    let first_part = &input[0..4];

    let result = format!("{}{}", second_part, first_part);
    result
}

fn stage3(byte_array: &Vec<u8>, xor_key: u8) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();

    for byte in byte_array.iter() {
        if *byte == xor_key {
            result.push(0x69)
        }
        else {
            result.push(*byte)
        }
    }

    result
}

fn encrypt(byte_array: Vec<u8>) -> Vec<u8> {
    let mut rng = thread_rng();
    let mut data = byte_array.to_vec();

    let seed: u16 = u16::from_be_bytes([data[0], data[1]]);;
    let random_number: u8 = rng.gen_range(10..=99);
    let shuffle_list = generate_unique_list(byte_array.len(), seed);

    data = stage1(data, random_number);
    data = stage2(&data, &shuffle_list);
    data = stage3(&data, random_number);

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

        let enc_data = encrypt(raw_data);

        if let Err(err) = write_bytes_to_file(&enc_data, encrypted_file) {
            println!("Error: {}", err);
        } else {
            println!("File encrypted successfully.");
        }
    }
    else {
        println!("usage: ./bomber <filename>");
    }
}
