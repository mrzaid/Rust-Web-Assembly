fn main(){
    let numbers : [i32 ; 7] = [20, 30, 25, 35, 16, 60, 100];
    //calculate sum of all array elements     
    let mut sum = 0;
    for a in 0..numbers.len(){
        sum = sum + numbers[a];
    }            
    //calculate average value
    let average = sum / numbers.len() as i32;
    println!("Sum of the array elements is : {}" , sum);
    println!("Average value of the array elements is : {}" , average);  
 
}
