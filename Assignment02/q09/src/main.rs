fn main(){
    let array: [i32; 5] = [8, 9, 3, 4, 5];
    let mut sum = 0; 
    println!("Find sum of all elements of array:");
    println!("----------------------------------"); 
    for n in 0..5{
        sum += array[n];     
    } 
    println!("Sum of all elements stored in the array is : {}", sum);
}
