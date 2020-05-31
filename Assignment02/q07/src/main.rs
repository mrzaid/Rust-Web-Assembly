fn main() {
    // display the cube of the number upto a given integer. 
    let x = 3;
    for i in 0..x{
        println!("Number is :{} and cube is :{}",i,(i as i32).pow(3) );
          //pow is method for power  //   (num as datatype).pow(power) 
    } 
}
