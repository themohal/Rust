#[derive(Debug)]
//use std::fmt;
struct Student {
    name:String,
    sub1:i32,
    sub2:i32 
}
fn main() {
let name = String::from("Online");
let sub1 = 55;
let sub2 = 77;
//tuple data
let student_tuple = (String::from("Online"),66,88);
println!("Name :{}",student_tuple.0 );
println!("Sub1 :{}",student_tuple.1 );
println!("Sub2 :{}",student_tuple.2 );

//index                             0       1   2
let student1 = Student{
   name: String::from("Online Student"),
   sub1:99,
   sub2:77, 
};
//struct1 data
println!("Name :{}",student1.name );
println!("Sub1 :{}",student1.sub1 );
println!("Sub2 :{}",student1.sub2 );
//struct2 data
let student2 = Student{
    sub2:77, 
    name: String::from("2nd Online Student"),
    sub1:85,
    
 };
println!("Name :{}",student2.name );
println!("Sub1 :{}",student2.sub1 );
println!("Sub2 :{}",student2.sub2 );
//to print whole struct
println!("Student 2 complete :{:?}",student2 );
println!("Student 2 complete :{:#?}",student2 );
//let address =format!("{:p}", &student2) as String;

//println!("address : {}",address);

//struct3 mut data
let mut student3 = Student{
    sub2:45, 
    name: String::from("3rd Online Student"),
    sub1:67,
    
 };
 println!("Student 3 complete :{:?}",student3 );
 student3.sub1 = 73;
 println!("Student 3 complete :{:?}",student3 );

//struct 4th mut data
let student4 = Student{
    sub1:43, 
    name: String::from("4th Online Student"),
    sub2:student3.sub1,
    
 };
 println!("Student 4 complete :{:?}",student4 );

 //struct 5th mut data
let student5 = Student{
    ..student4
    
 };
 println!("Student 5 complete :{:?}",student5 );
}
