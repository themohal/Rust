# Even Odd Crate
this is a demo rust library published on crates.io

to use this library you have to add following line in dependency section of cargo.toml

`even_odd = "0.1.1"`

your cargo.toml file should look like this:
```
[package]
name = "even_odd"
version = "0.1.0"
authors = ["farjadali <farjadmohal@gmail.com>"]
edition = "2018"

[dependencies]
even_odd_crate = "0.1.1"
```

In `src/lib.rs` you can use like this:
```
This function will check the Input:
```
To use in main.rs
```
```
use even_odd;
fn main(){
even_odd::even_odd();
}
```
This function will check the Input:
```
pub fn even_odd(){
        println!("******Welcome to Even Odd Rust Program******");
        println!("Please Enter A Number To Check: " );
       
        let mut num = String::new();
        io::stdin().read_line(&mut num).expect("Error");
        
        let check_num = num.trim();
        match check_num.parse::<u32>() {
            Ok(i) => {
                println!("Your integer input: {}", i);
                let num_to_int:u32 = check_num.parse().unwrap();
            if num_to_int %2 ==0 {
                println!("Number is Even");
        
            }
            else {
                println!("Number is Odd");
        
          }
        }
            ,
            Err(..) => println!("This was not an integer: {}", check_num),
        };
       
    }
```


now `cargo run` for results
