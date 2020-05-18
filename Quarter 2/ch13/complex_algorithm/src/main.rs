use std::time::Duration;
use std::thread;
fn main() {
    let simulated_user_specified_value=10;
    let simulated_rand_num=3;
    generate_workout(simulated_user_specified_value,simulated_rand_num);
}

fn generate_workout(intensity:u32,random_number:u32){
    // lets make it more efficient by using closure function and remove simulated_expensive_calculation function 
    let expenisve_result = |num|{
        println!("Calculating slowly");
        thread::sleep(Duration::from_secs(2));
        num
    };
    if intensity < 25{
        println!("today do {} pushups",expenisve_result(intensity));

        println!("next do {} situps",expenisve_result(intensity));
    }
    else{
        //this block is to be run then we are wasting time 2 sec above getting value
        if random_number ==3 {
            println!("take a break today");
        }
        else{
            println!("Today run for {} mints",expenisve_result(intensity));
        }
    }
}