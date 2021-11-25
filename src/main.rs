use std::io;

mod password;

fn main() {
    println!("Let's generate a random password for you.");
    println!("Enter the length of required password: ");
        let mut length = String::new();
        io::stdin()
            .read_line(&mut length)
            .expect("Failed to read line");
        let length: u32 = length.trim().parse().expect("Please Enter a number");

    match password::generate(length) {
        Ok(pass) => println!("{}", pass),
        Err(err) => println!("{}", err)
    };
}
