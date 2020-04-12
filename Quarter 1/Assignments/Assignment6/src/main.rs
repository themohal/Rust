#[derive(Debug)]
struct Student{
    name:String,
    subject1:u8,
    subject2:u8,
}
fn main() {
    let student_1= Student {
        name : "Farjad".to_string(),
        subject1 : 75,
        subject2 : 85,
    } ;
    println!("Complete Instance : {:#?}",student_1 );

    println!("Printing Instance by Calling Fields :  {},{},{}",student_1.name,student_1.subject1,student_1.subject2 );
    
    let student_2 = Student {
        name : student_1.name,
        subject1 : student_1.subject1,
        subject2 : student_1.subject2,
    };
    println!("Other Instance Student 2 Created: {:#?}",student_2);

    let ret_inst = u_def_fun("Noman".to_string(),76,88);
    println!("Returned Instance from Function: {:#?}",ret_inst );
}
fn u_def_fun(name:String,sub1:u8,sub2:u8)-> Student {
Student{
    name:name,
    subject1:sub1,
    subject2:sub2,
}
}