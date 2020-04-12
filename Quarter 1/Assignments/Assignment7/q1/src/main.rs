mod school {
    pub mod student {
        pub fn print_attendance(){
            println!("Hello , I am Present");
        }
    }
    }
    
use crate:: school::student;

fn main() {
    student::print_attendance();
}
