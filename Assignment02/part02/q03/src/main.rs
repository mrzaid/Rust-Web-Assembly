fn main() {
    let num = 100;
    match num{
        10 => println!("decade"),
        100 => println!("Century"),
        1000 => println!("Millenium"),
        _ => println!("please enter from 10, 100 or 1000"),
    }
}
