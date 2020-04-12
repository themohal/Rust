
    #[derive(Debug)]
    //use std::fmt;
    struct Student {
        name:String,
        sub1:i32,
        sub2:i32 
    }
    // struct type tuple
    struct Color(i32,i32,i32);
    fn main() {
        let col1=(32,32,32);
        println!("{:#?}",col1 );
    let name = String::from("Online");
    let sub1 = 55;
    let sub2 = 77;
    //tuple data
    let student_tuple = (String::from("Online"),66,88);
    println!("Name :{}",student_tuple.0 );
    println!("Sub1 :{}",student_tuple.1 );
    println!("Sub2 :{}",student_tuple.2 );
    
    let returned_student = variable(name, sub1, sub2);
    println!("Returned Value in Main : {:#?}",returned_student );
    tuple(student_tuple);
    //index                             0       1   2
    let student1 = Student{
       name: String::from("Online Student"),
       sub1:99,
       sub2:77, 
    };
    
    //struct1 data
    println!("Student 1 complete :{:?}",student1 );
    let add=struct_func(student1);
       println!("Result of Addition :{}",add );
    
fn variable(name:String,sub1:i32,sub2:i32)-> Student {
    println!("We are in variable function");
    println!("{} {} {}",name,sub1,sub2);
    let student4 = Student {
        name,
        sub1,
        sub2
    };
    
     

    println!("Student 4 :{:#?}",student4);
    let return_student= student4;
    //constructor
    //Student{
      //  name, 
        //sub1,
        //sub2
    // }
    return_student
    
}    
fn tuple(student_data:(String,i32,i32)){
    println!("We are in tubple function");
    println!("{} {} {}",student_data.0,student_data.1,student_data.2);
}

fn struct_func(student2:Student)-> i32 {
    println!("We are in Struct function");
    println!("{:?}",student2);
    student2.sub2+student2.sub1

}

    }


