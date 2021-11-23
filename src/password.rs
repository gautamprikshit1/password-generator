use rand::Rng;
use std::io;

pub fn generate() {
    let chars: String = String::from("!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~");
    let mut pass: String = String::new();
    loop {
        println!("Enter the length of required password: ");
        let mut length = String::new();
        io::stdin()
            .read_line(&mut length)
            .expect("Failed to read line");
        let length: u32 = length.trim().parse().expect("Please Enter a number");
        if length < 8 {
            println!("Way too short for a password! try again.");
        } else {
            for _i in 0..length {
                let index = rand::thread_rng().gen_range(0..chars.len());
                pass.push(chars.chars().nth(index).unwrap());
            }
            println!("Your required password is {}", pass);
            break;
        }
    }
}
