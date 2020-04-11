use std::io;
use std::fs;


fn read_username_from_file()-> Result <String,io::Error>{
    fs::read_to_string("hello.txt")
}
fn main() {
let data = read_username_from_file();
   // println!("{:?}",data);

    let name = match data {
        Ok(a) => a.trim().to_string(),
        Err(_) =>String::from("Error"),


    };
    println!("{:#?}",name );
}