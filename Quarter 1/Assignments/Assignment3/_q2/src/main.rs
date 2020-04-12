//Main function starts from here
fn main() {

    let mut counter = 1;  //decalaring and intializing a mutable variable
    while counter <= 10 {      //start of while loop
        
        if counter == 3 {     //check
        println!("Warning ! Special Security Check");
        }
        if counter == 7 {     //check
        println!("Warning ! Special Security Check");
        }
        if counter == 10 {    //check     
        println!("Warning ! Special Security Check");
        }    
        println!("{}",counter); //priting the counter on screen
        counter = counter +1;   //incrementing counter value
        }
    }//End of Main