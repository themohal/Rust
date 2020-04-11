use std::io;

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