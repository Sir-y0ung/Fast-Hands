use regex::Regex;

const REGEX_VALIDATE:[&str;9] = [r"^.{33}$", r"(?:^[\x48][\x54][\x42]).*", r"^.{3}(\x7b).*(\x7d)$", r"^[[:upper:]]{3}.[[:upper:]].{3}[[:upper:]].{3}[[:upper:]].{3}[[:upper:]].{4}[[:upper:]].{2}[[:upper:]].{3}[[:upper:]].{4}$", r"(?:.*\x5f.*)", r"(?:.[^0-9]*\d.*){5}", r".{24}\x54.\x65.\x54.*", "^.{4}[X-Z]\\d._[A]\\D\\d.................[[:upper:]][n-x]{2}[n|c].$", ".{11}_T[h|7]\\d_[[:upper:]]\\dn[a-h]_[O]\\d_[[:alpha:]]{3}_.{5}"];

fn filter_input(user_input:&str) -> i32 {
    for i in REGEX_VALIDATE {               
        let re = Regex::new(i).unwrap();    // looping all over to match input
        if !re.is_match(&user_input) {
            return 0;
        }
    }
    return 1
}

fn multiply_characters(input: &str) -> u128 {
    input.chars()  // Iterate over each character in the string
        .map(|c| c as u128)  // Convert each character to its numeric value (Unicode scalar value)
        .product()  // Multiply all values together
}

fn check_input(user_input:String) -> i32 {
    let slices:[&str; 7] = [&user_input[4..7], &user_input[8..11], &user_input[12..15], &user_input[16..20], &user_input[21..23], &user_input[24..27], &user_input[28..32]];
    let corr_values:[u128;7] = [0x7a070, 0x5c436, 0x6cc60, 0x27b5776, 0x10f9, 0xd76a0, 0x7465a58];
    let mut c:usize = 0;

    for i in slices {
        let givven = multiply_characters(i);    // Check if conditions are met 
        if givven != corr_values[c] {
            return 0
        }
        c += 1
    }
    1
} 

fn main() {
    let mut user_input = String::new();

    println!("Welcome to our secret town!");
    println!("Enter secret passphrase:");
        
    let b1 = std::io::stdin().read_line(&mut user_input).unwrap();

    let trimmed_input = user_input.trim_end();
    
    let mut result = filter_input(&trimmed_input);

    result &= check_input(user_input.to_string());
    
    if  result != 0 {
        println!("Correct one of us!!");
    }   
    else {
        println!("Maybe next time :<");
    }
}
