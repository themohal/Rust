use std::fmt::Display; //used for generic 
//#[derive(Debug)] used for enums and structs printing
fn main(){ //avoiding duplication

    let password1 = String::from ("Karachi");
    //check_password(1111);
    //check_password(2222); //same function is priting diff concrete values
    //string_check_password(password1);
    check_password_generic(password1);
    check_password_generic(1111);
    check_password_generic(22.22);
    check_password_generic('a');
}
fn check_password(x:i32){
println!("{}",x );
}
fn string_check_password(x:String){
    println!("{}",x );
    }
    //these both functions are duplication both printing pass
    //so to avoid this we use generics
    //function should get diff conrete values of different data types
    //<T> this tells x is of anytype T means generic
    //T:Display is for printing
fn check_password_generic<T:Display>(x:T){
    println!("{}",x );
}