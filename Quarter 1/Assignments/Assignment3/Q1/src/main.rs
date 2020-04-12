use std::io;
//Main function starts from here
fn main() {
choice();           //function call to make choice
}
//End of Main
fn choice(){         //Declaring choice function
    let mut choice = String::new();  //declaring and initializing new string variable
    let parsed_choice:u8;              //declaring variable to store parsed value from string
        println!("Welcome to BMI Calculator");   //printing on screen
        println!("1. Metric BMI");
        println!("2. Imperial BMI");
        println!("Please choose which BMI you want calculate and hit Enter :");

            io::stdin().read_line(&mut choice) //to read from input and then store to the memory address pointed
            .ok()                              //if everything is ok
            .expect("Error reading choice !"); //otherwise throw error message
            parsed_choice = choice.trim().parse().expect("Error parsing"); //trimming white spacing and parsing string variable
        if  parsed_choice ==1 {           //decision making
            metric_bmi();                 //calling metric BMI function
        }
        else if parsed_choice ==2{
            imperial_bmi();                //calling imperial BMI function
        }
    }
fn metric_bmi(){ //Declaring Metric BMI function
    let mut input_height = String::new();      //declaring string variables and initializing
    let mut input_weight = String::new();
    let parsed_height:f64;                     //declaring variables to store parsed value from string
    let parsed_weight:f64;

        println!("Enter Height in Cm :");   
            io::stdin().read_line(&mut input_height)  //to read from input and then store to the memory address pointed
            .ok()
            .expect("Failed to read line");
        println!("Enter Weight in Kgs :");
            io::stdin().read_line(&mut input_weight) //to read from input and then store to the memory address pointed
            .ok()                                   //to check if everything is ok
            .expect("Failed to read line");           //throwing error message
        //println!("Your height : {}", input_height);
        //println!("Your weight : {}", input_weight);
                parsed_height = input_height.trim().parse().expect("Error"); //parsing from string
                parsed_weight = input_weight.trim().parse().expect("Error");    //parsing from string
            
                     //println!("Parsed height: {}", parsed_height);
                     //println!("Parsed weight: {}", parsed_weight);

//BMI Metric 
    let mut conv_height:f64 = parsed_height/100.0;  //declaring new variable an initializing it with calculated value
        conv_height = conv_height*conv_height;      //taking square
    let metric_bmi = parsed_weight/conv_height;     //declaring variable and initializing it with result
        println!("Metric BMI is : {:.2}", metric_bmi);  //:.2 indicates that print the floating number upto 2 decimal rounded up values
        if metric_bmi < 18.5{             //decision making
        println!("You are Underweight");
        }
        else if metric_bmi >18.5 && metric_bmi < 25.0 {
            println!("You are Normal Weight");
        }
        else if metric_bmi > 25.0{
        println!("You are Overweight");
        }
    } //end Metric BMI function

fn imperial_bmi(){ //Declaring Imperial BMI function
//BMI Imperial Code Starts From Here
    let mut input_height_feet = String::new();  //declaring and initializing strings
    let mut input_inches = String::new();
    let mut input_weight_pounds = String::new();
    let parsed_height_feet:f64;                //declaring variables to store parsed values
    let parsed_height_inches:f64;
    let parsed_weight_pounds:f64;

        println!("Enter Your Height Feet :");
            io::stdin().read_line(&mut input_height_feet) //to read from input and then store to the memory address pointed
            .ok()                                        //to check if everything is ok
            .expect("Failed to read line");             //throwing error message
        println!("Enter Your Height inches :");
            io::stdin().read_line(&mut input_inches) //to read from input and then store to the memory address pointed
            .ok()                                     //to check if everything is ok
            .expect("Failed to read line");         //throwing error message
            println!("Enter Weight in Pounds :");
            io::stdin().read_line(&mut input_weight_pounds) //to read from input and then store to the memory address pointed
            .ok()                                            //to check if everything is ok
            .expect("Failed to read line");                 //throwing error message
        //println!("Your height : {}.{}", input_height_feet,input_inches);
        //println!("Your weight : {}", input_weight_pounds);
            parsed_height_feet = input_height_feet.trim().parse().expect("Error");  //Parsing values from strings
            parsed_height_inches = input_inches.trim().parse().expect("Error");
            parsed_weight_pounds = input_weight_pounds.trim().parse().expect("Error");    
          //          println!("Parsed height: {}.{}", parsed_height_feet,parsed_height_inches);
            //        println!("Parsed weight: {}", parsed_weight_pounds);
    let mut conv_height_inches:f64 = parsed_height_feet*12.0;   //declaring and initializing variable to convert feet into inches
        conv_height_inches =conv_height_inches+parsed_height_inches; //adding extra inches of height
        conv_height_inches = conv_height_inches*conv_height_inches; //taking square of height
    let imp_bmi = 703.0*(parsed_weight_pounds/conv_height_inches); //calculating result
        println!("Imperial BMI is : {:.2}", imp_bmi); //:.2 indicates that print the floating number upto 2 decimal rounded up values
        if imp_bmi < 18.5{ //decision making
            println!("You are Underweight");
        }
        else if imp_bmi >18.5 && imp_bmi < 25.0 {
            println!("You are Normal Weight");

        }
        else if imp_bmi > 25.0{
            println!("You are Overweight");
        }
    }//End of Imperial BMI Function