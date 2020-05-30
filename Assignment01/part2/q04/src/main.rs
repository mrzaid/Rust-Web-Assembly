fn main() {
    let a = 235;
    let b : i32 = 35423;
    let c : i64 = 03204169066;
    let d : i16 = 1112;
    let e : f32 = 4.65123;
    let f : f64 = 2.28964926;
    let g = 'A';
    let h : u64 = 03314652335;
    println!("a + g is {}",a as u32+g as u32);
    println!("e + g is {}",e as u32 +g as u32);
    println!("f + h is {}",f+h as f64);
    println!("a + h is {}",a+h);
    println!("d + b is {}",d as i32 +b);
    println!("c + b is {}",c+b as i64);
    println!("d + g is {}",d as u32 +g as u32);
    println!("c + g is {}",c as u32 +g as u32);
    println!("c + h is {}",c as u64 +h);
}
