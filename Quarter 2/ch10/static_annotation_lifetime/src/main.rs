
fn main() {
let s:&'static str = "Hello world";  //&'static lifetime parameter tells references to exist through out the program.
//string literals have this 
//this will not outlive
println!("{}",s );
}
