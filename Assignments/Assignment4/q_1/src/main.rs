fn main() {

    check_number(-1); //function call
}
fn check_number(number:i32){ //function signature
    //function definition starts from here
    //check the number with if condition
    if number < 0 {
        println!("Number is negative");
    }
    else if number == 0 {
        println!("Number is equal to zero");
    }
    else {
        println!("Number is positive");
    }
}//end of program
