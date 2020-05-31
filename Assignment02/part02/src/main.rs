fn main() {
    let num = 11;
    let mut c = 0;
    for i in 1..num+1{
        if num % i == 0{
            c += 1;
        }
    }
    if c >= 2{
        println!("The number is prime")
    }
    else{
        println!("The number is not prime")
    }
    
}
