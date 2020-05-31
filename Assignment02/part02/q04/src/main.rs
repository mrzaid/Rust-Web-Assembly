fn main() {
    let mut sum = 0;
    println!("The first ten natural numbers are \n 1 2 3 4 5 6 7 8 9 10");
    for n in 1..11{
        sum += n;
    }
    println!("The sum is {}", sum);
}
