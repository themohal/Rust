use std::io;
fn main() {
    let mut inputdata = String::new();
    println!("Enter A Number: ");
    io::stdin().read_line(&mut inputdata).ok().expect("Failed to readline");
    let int_data :u32 = inputdata.trim().parse().expect("Please type number!");

    if int_data==1{
        println!("One");
    }else if int_data == 2 {
        println!("two");

    }
    else if int_data == 3 {
        println!("Three");

    }
    else{
        println!("Not in list");

    }
    println!("Here is from Match");
    match int_data {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Not In the list"), 
    }
}
