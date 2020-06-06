// #[Derive(Debug)]
// struct Book{
//     name : String,
//     author : String,
//     price : u16,
//     availability : bool,
// }//template we just defined here a custom data type that is Book
// fn main() {
//     let book_1 = Book {
//         name : String::from("Book A"),
//         author : String::from("Author A"),
//         price : 500,
//         availability:true,
//     };// yeh poori book_1 ka samaan bataya hai
//     let book_2 = Book {
//         name : String::from("Book B"),
//         author : String::from("Author B"),
//         price : 600,
//         availability:true,
//     };
//     let book_3 = Book {
//         name : String::from("Book C"),
//         author : String::from("Author C"),
//         price : 700,
//         availability:true,
//     };
    
// println!("{:#?}",book_1);
// println!("{:#?}",book_2);
// println!("{:#?}",book_3);    
// }

                             //MUTABLE STRUCT
// #[derive(Debug)]
// struct Book{
//     name : String,
//     author : String,
//     price : u16,
//     availability : bool,
// }//template we just defined here a custom data type that is Book
// fn main() {
//     let mut book_1 = Book {
//         name : String::from("Book A"),
//         price : 500,
//         availability:true,
//         author : String::from("Author A"),//INITIALIZATION KAY WAQT AGAY PECHAY NP :P
//     };
//             book_1.name = String::from ("Book A9");
// println!("{:#?}",book_1);   
// }

        //FIELD INIT SHORTHAND
// #[derive(Debug)]
// struct Book{
//     name : String,
//     author : String,
//     price : u16,
//     availability : bool,
// }
// fn main(){
//     let book_1 = build  ( String::from("BOOK A"), String::from("AUTHOR A") );
//     // build mein dalay ga value
//     println!("{:#?}",book_1);
// }
// fn build (name:String,author:String)->Book{
//     Book{
//      name,//name:  KEY AND PARAMETER NAME SHOUD BE SAME
//      author, //author:
//     price: 500,
//     availability: true,
//     }
// }
                // UPDATE SYNTAX
// #[derive(Debug)]
// struct Book{
//     name : String,
//     author : String,
//     price : u16,
//     availability : bool,
// }//template we just defined here a custom data type that is Book
// fn main() {
//     let book_1 = Book {
//         name : String::from("Book A"),
//         author : String::from("Author A"),
//         price : 500,
//         availability:true,
//     };// yeh poori book_1 ka samaan bataya hai
//     let book_2 = Book {
//         name : String::from("Book B"),
//         author : String::from("Author B"),
//         price :book_1.price,
//         availability:book_1.availability,
//     };
//     let book_3 = Book {
//         name : String::from("Book C"),
//         author : String::from("Author C"),
//         ..book_1 //coma ni lagatay bhens
//     };
    
// println!("{:#?}",book_1);
// println!("{:#?}",book_2);
// println!("{:#?}",book_3);    
// }
            //TUPPLE STRUCT
// #[derive(Debug)]
// struct Colors (i32,i32,i32);
// struct Points (i32,i32,i32);

// fn main(){
//     let black = Colors(5,6,7);
//     another_function(black);
// }
// fn another_function (x:Colors){
//     println!("{:#?}",x);
    
// }
       //STRUCT OWNERSHIP
           //REFACTORING WITH STRUCTS
// #[derive(Debug)]
// struct Rectangle{
//     height:u32,
//     width:u32,
// }
// fn main(){
// let rec_1=Rectangle{
//     height:100,
//     width:50,
// };
// println!("The area of rectangle is:{}",area(&rec_1));
// println!("{:#?}",rec_1);
// }
// fn area(rec:&Rectangle)->u32{
//     rec.height*rec.width
// }
      //USER INPUT
use std::io;
fn main(){
    let mut s= String::new();
    println!("please dalo kuch {}",s);
    io::stdin().read_line(&mut s).expect("failed");
    println!("yeh lo {}",s)
}
