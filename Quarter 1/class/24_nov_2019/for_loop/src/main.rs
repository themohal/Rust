fn main() {
    for design in 1..=4 { //put =4 if u wanna include 4 then 1..=4
        println!("Design # is : {}",design );
    }
    println!("Printing in reverse order");
    for design in (1..=4).rev() { //put =4 if u wanna include 4 then 1..=4
        println!("Design # is : {}",design );
    }
    //for array
    let age:[f32;5] = [10.1,20.2,30.3,40.4,50.5]; //to use power or pow function we need to define data type and size of array for floating use powi function
    for data in age.iter(){
        println!("Age is : {}",data.powi(2) );
    }
    println!("End of Program");
}
