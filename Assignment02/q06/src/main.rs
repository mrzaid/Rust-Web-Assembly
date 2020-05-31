use std::io;
fn main(){
    let mut sum=0;
    for i in 0..10{
        let mut data = String::new(); 
        println!("Enter integer {}",i+1);
        io::stdin().read_line(&mut data);      
        let mut data : i32 = data.trim().parse().unwrap(); 
        sum = sum + data;
    }    
    println!("The sum is: {}",sum);
    println!("The average is: {}",sum/10); 
}
