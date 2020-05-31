fn main() {
    let mut a = 0;
    loop{
        if a<3{
        println!("I love my mother");
        a += 1;
        }
        else{
            println!("I love my Father");
            break;
        }   
    }


    for i in 1..5{
        if i<4{
            println!("I love my mother");
        }
        else{
            println!("I love my Father");
        }
    }
    

    while a<6{
        println!("I love my Mother");
        a += 1;
    }
    println!("I love my Father");
}
