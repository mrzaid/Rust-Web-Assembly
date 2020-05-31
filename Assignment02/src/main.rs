use std::{io};

fn main(){               
    
    let mut int = String::new();
    io::stdin().read_line(&mut int)
        .expect("Failed to read your input");
    
    let int: i32 = match int.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };
    println!("{}",factorial(int));
    //is_even_odd(int); 
    // Enter your code below this line.
}

fn factorial(number:i32)->i32{
    let mut fact : i32 = 1;
    for n in 1..number+1{
        fact = n * fact;
    }
    fact

// fn is_even_odd(x:i32)->String {
    
//     if x % 2 == 0 {
//         println!("Even");
//     }
//     else{
//         println!("Odd");
//     }
//     x.to_string()
}
