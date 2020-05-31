fn main(){
    let names = ["Ali", "Zain", "Naufil"];
    for name in names.iter(){
        match name{
            &"Ali" => println!("There is a rockstar among us! Hello {}", name),
            &"Zain" => println!("There is a musician among us! Hello {}", name),
            &"Naufil" => println!("There is a athlete among us! Hello {}", name),
                 _ => println!("Hello {}", name),     
        } 
    }
}
