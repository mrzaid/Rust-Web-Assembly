


use std::io;

pub fn main() {
    let number = input("Enter an input");
    println!("{}", to_binary(number));
}

fn input(msg: &str) -> i32 {
    println!("{}", msg);
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read input");

    match number.trim().parse() {
        Ok(num) => num,
        Err(_e) => panic!("Not a number"),
    }
}

fn to_binary(mut decimal: i32) -> i32 {
    if decimal == 0 {
        decimal
    } else {
        let mut bits = String::new();

        while decimal > 0 {
            if decimal % 2 == 0 {
                bits.push_str("0");
            } else {
                bits.push_str("1");
            }

            decimal /= 2;
        }
        // reverse the bits
        match bits.chars().rev().collect::<String>().parse() {
            Ok(num) => num,
            Err(_e) => panic!("Something went wrong"),
        }
    }
}