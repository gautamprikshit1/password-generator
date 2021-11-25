use rand::Rng;

pub fn generate(length: u32) -> Result<String, &'static str> {
    let chars: String = String::from("!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~");
    let mut pass: String = String::new();
    if length < 8 {
        return Err("Password length should be greater than 8");
    } else {
        for _i in 0..length {
            let index = rand::thread_rng().gen_range(0..chars.len());
            pass.push(chars.chars().nth(index).unwrap());
        }
    return Ok(format!("Your required password is {}", pass));
}
}
