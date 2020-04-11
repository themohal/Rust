use std::io;

fn main() {
    let mut name = String::new();  //declaring and initializing new string variable
    let mut s1_mark = String::new();
    let mut s2_mark = String::new();
    let parsed_mark1:f32;
    let parsed_mark2:f32;              //declaring variable to store parsed value from string
        println!("Welcome to Percentage Calculator");   //printing on screen
        println!("Please Enter Your Name: ");
        io::stdin().read_line(&mut name) //to read from input and then store to the memory address pointed
            .ok()                              //if everything is ok
            .expect("Error reading choice !"); //otherwise throw error message
            println!("Please Enter Subject 1 Marks: ");
            io::stdin().read_line(&mut s1_mark) //to read from input and then store to the memory address pointed
            .ok()                              //if everything is ok
            .expect("Error reading choice !"); //otherwise throw error message
            parsed_mark1 = s1_mark.trim().parse().expect("Error parsing"); //trimming white spacing and parsing string variable
            println!("Please Enter Subject 2 Marks: ");
            io::stdin().read_line(&mut s2_mark) //to read from input and then store to the memory address pointed
                .ok()                              //if everything is ok
                .expect("Error reading choice !"); //otherwise throw error message
            parsed_mark2 = s2_mark.trim().parse().expect("Error parsing"); //trimming white spacing and parsing string variable
        let returned_percentage=calc(name, parsed_mark1, parsed_mark2); //parsing and returning percentage
       //checking conditon
        if returned_percentage >=70.0 {
            println!("Pass" );
        } 
        else if returned_percentage < 70.0 {
            println!("Fail")
        }
    }//end of main
fn calc(s_name:String,sb1_marks:f32,sb2_marks:f32)->f32{//function signature
    println!("*** Detail of Student ***");
println!("Name: {}",s_name );
println!("Subject 1 Marks: {}",sb1_marks );
println!("Subject 2 Marks: {}",sb2_marks);
let obtained_marks:f32= (sb1_marks+sb2_marks)/200.0;
let percentage = obtained_marks*100.0;
println!("Percentage: {}",percentage );
percentage


}//end of calc function
