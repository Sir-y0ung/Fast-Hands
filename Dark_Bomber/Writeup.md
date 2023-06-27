Challenge Name:
-----------------
Dark Bomber

Discription:
-----------------
It's the year 2244. Time to leave the planet.
Eighty years ago, Earth faced a crisis like never before. Fossil fuels had been exhausted, water was scarce, and power a luxury. In a world filled with darkness, tensions began to rise between nations, and the once united global community split into two power states, each vying for control, power, and survival. These 80 years of dispute stayed in history as the Dark War.

Now, 16 years after the Armistice, warface is about to re-emerge. Scientists have confirmed an alternative power source on Mars – Vitalium – and the two states are now racing to establish a colony on the red planet. A ruthless competition of hacking and cyber-attacks has begun, and both sides are trying to damage and exploit each other's infrastructure.

You're part of the United Nations of Zenium, and your state wants to create a democratic colony where Vitalium will be used to make humanity healthy again. Your opponent is the Board of Arodor, a strict dictatorship that wants to ensure only the elite can get a second chance in life.

Win and escape the planet or fail to survive.


Analyzing the binary
----------------------
Running strings we can easily identify that is a 64 bit rust binary

![rsproof](https://github.com/YoungFlexerGR/challDev/assets/82509480/59216039-ff39-4c71-a405-ad79de3fdbbc)

Opening the executable in ida seems that is not stripped so we have all the function names :)
Lets jump into main to start reversing the binary.Seems that the binary takes one comand line argument where it is the file to encrypt
we see that there is a function encrypt that takes as argument the bytes from the given filename 

![encrypt](https://github.com/YoungFlexerGR/challDev/assets/82509480/16791c62-6b3e-45da-ae40-92902c673bec)

Analyzing encrypt function a bit more
we can see that there is 3 stages of encryption
![encryptFunction](https://github.com/YoungFlexerGR/challDev/assets/82509480/3a52bb4f-35a1-43e2-80b7-13db67f2c807)


Stage1
----------------------
So stage1 get as arguemnt the raw bytes from the given file and a random key in range 10 - 99.
After that the function iterates all the given bytes and call function `level1` which is a simple 
implementation of xor operation

![stage1](https://github.com/YoungFlexerGR/challDev/assets/82509480/40136a53-d41f-41e2-a6ce-bfba5f6bd13b)


Stage2
----------------------
Stage2 gets as arguments the bytes that xored on stage1 and an array that generated from `generate_unique_list`
function.`Generate_unique_list` in total generates a unique list with a given seed to shuffle the xored bytes.
program seems to give as seed the first 2 bytes from the given file

![stage2](https://github.com/YoungFlexerGR/challDev/assets/82509480/357aa795-50cd-4081-866d-feeb6159a7f5)

Stage3
----------------------
Finally stage 3 iterates all xored-shuffled bytes and replace the bytes that have the same value as xor_key with
0x69

![stage3](https://github.com/YoungFlexerGR/challDev/assets/82509480/179d6cf2-1714-41eb-83dc-011bf278e6c9)

Solver
----------------------
So for now, we have to write a decryption script to recover bombDisposalKit.elf.enc.
First of all, we have to unshuffle the encrypted bytes, then find the xor_key , replace 0x69 with the xor_key 
and xor bytes.

rust Solver

```rust
use std::fs::File;
use std::io::{Read, Write};
use rand::seq::SliceRandom;
use rand::{Rng, thread_rng};
use rand::rngs::StdRng;
use rand::SeedableRng;

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

fn generate_unique_list(n: usize, seed: u16) -> Vec<usize> {
    let mut unique_list: Vec<usize> = (0..=n).collect();
    let mut rng = StdRng::seed_from_u64(seed as u64);
    unique_list.shuffle(&mut rng);
    unique_list
}

fn find_xor_key(encdata: u8) -> u8 {
    let xor_key: u8 = encdata ^ 0x7f;
    xor_key
}

fn decrypt_st_1(enc_data: Vec<u8>, xor_key: u8) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();

    for i in 0..enc_data.len() {
        result.push((enc_data[i] ^ xor_key) as u8)
    }

    result
}


fn unshuffle_st_2(enc_data: Vec<u8>) -> Vec<u8> {
    let mut shuffled_list = generate_unique_list(enc_data.len() - 1, 0x7F45);
    let mut result: Vec<u8> = vec![0; shuffled_list.len() -1];

    for i in 0..(shuffled_list.len() - 1) {
        result[i] = enc_data[shuffled_list[i]];
    }
    result
}

fn decrypt_st_3(enc_data: Vec<u8>, xor_key: u8) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();

    for i in 0..enc_data.len() {
        if enc_data[i] == 0x69 {
            result.push(xor_key);
        }
        else {
            result.push(enc_data[i]);
        }
    }
    result
}

fn main() {
    let mut data: Vec<u8> = Vec::new();
    let filename = "bombDisposalKit.elf.enc";
    let mut output = String::from(filename);
    output.drain(output.len() - 3..);

    match read_file_to_bytes(filename) {
        Ok(bytes) => {
            println!("Trying to decrypt file");
            data = bytes;
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }

    data = unshuffle_st_2(data);
    let xor_key: u8 = find_xor_key(data[0]);
    println!("Found Key: {}", xor_key);
    data = decrypt_st_3(data, xor_key);
    data = decrypt_st_1(data, xor_key);

    if let Err(err) = write_bytes_to_file(&data, output) {
        println!("Error: {}", err);
    } else {
        println!("File decrypted successfully.");
    }
}
```
And Boom here is your Flag:

![flag](https://github.com/YoungFlexerGR/challDev/assets/82509480/87f249e6-1d57-481a-8f62-92a6ea485d28)
