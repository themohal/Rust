fn main() {
    let tuple =value_send_receive_square(15); //function calling and storing returned value in tuple
    println!("Received values are: {:?}",tuple); //printing tuple
    //seprating values
    let number = tuple.0;
    let square = tuple.1;
    //printing values separately
    println!("Number sent and returned is: {}",number);
    println!("Sqaure of number returned is: {}",square);    

}
fn value_send_receive_square(number:u16)-> (u16,u16) { //function signature when we define the returning values in more then 2 it will need tuple or array
    (number,number*number) //this will return a tuple 
    
}//end of program
