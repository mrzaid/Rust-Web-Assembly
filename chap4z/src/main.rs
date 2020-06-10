               ///push something in heap and clone
// fn main() {
//     let mut x= String::from("hello");
//     x.push_str(" boi");
//     println!("{}",x);
//     let s1= String::from("hello");
//     let s2=s1.clone();
//     println!("{},{}",s1,s2);
// }
                /// copy of stack and heap(not copied)
// fn main(){
// let s = String::from("Hello");//s comes intop scope
// takes_ownership(s.clone());// s is moved to fn owner wala
// println!("heap bhi copy mamo {}",s);
// let z=899;
// makes_copy(z);
// println!("stack ka maal copy{}",z);
// }
// fn takes_ownership(x:String){//x comes intop scope
//     println!("{}",x);
// }
// fn makes_copy(y:i32){
//     println!("{}",y);
// }
           //RETURNING VALUES AND SCOPE
// fn main(){
// let result1=gives_ownership();// catch return value from fn gives owner
// println!("han bol {}",result1);
// let mut x =String::from("jiye nepal");
// let result2=takes_and_gives_back(x);// x is moved to takes and gives back
// println!("final {}",result2);

// }
// fn gives_ownership()->String{
//     let s= String::from("jiye bhutto");
//     s// this return value will go and sit in result and we will print
// }
// fn takes_and_gives_back(mut x:String)->String{
//     x.push_str("i londa");
//     x

// }
             //RETURNING MULTIPLE VALUES
// fn main(){
//     let s= String::from("han bhai keisa diya");
//     let (result,result1)=length(s);  // 2 container
//     println!("The Length of Word {} is {}",result1,result);
    
// }
// fn length(name:String)->(usize,String){
//     (name.len(),name)
// }
           //BORROWING
// fn main(){
//  let   a:u8 = 10;
//  let b=&a;
//  let c=&a;
//  println!("a:{} b:{} c:{}",a,b,c);
//  println!("address of a is {:p}",b);
// println!("address of b is {:p}",c);
// }
      //referencing
// fn main(){
//     let s= String::from("PAKISTAN");
//     let result=length(&s);  // 2 container
//     println!("The Length of Word {} is {}",s,result);
    
// }
// fn length(name:&String)->usize{
//     name.len()
// }
     //mutable reference
// fn main(){
//     let mut s=String::from("Jiye ");
//     khel(&mut s)
// }
// fn khel(x:&mut String){
//     x.push_str("Bhutto");
//     println!("{:?}",x);
// }