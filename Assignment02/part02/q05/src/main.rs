use std::io;
fn main() {
    
    println!("{}",eligibility());
}
fn eligibility()->String{
    let tot_marks = 180;
    println!("input the marks obtained in Physics");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1);
    let input1:i32 = input1.trim().parse().unwrap();

    println!("input the marks obtained in Chemistry");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2);
    let input2:i32 = input2.trim().parse().unwrap();

    println!("input the marks obtained in Mathematics");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3);
    let input3:i32 = input3.trim().parse().unwrap();

    let obt_marks = input1 + input2 + input3;

    if obt_marks >= 180{
        "The candidate is eligible for admission".to_string()
    }
    else{
        "The candidate is not eligible for admission".to_string()
    }


    
}