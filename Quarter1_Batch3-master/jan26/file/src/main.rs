use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f  = File::open("hello.txt");
    println!("{:#?}",f);

    let f = match f {
        Ok(file) => file,
        Err(xyz) => match xyz.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
    println!("{:#?}",f);
}