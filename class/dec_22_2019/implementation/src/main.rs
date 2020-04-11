#[derive(Debug)]
struct Student {
name:String,
score:u8,
}
struct Score {
    score1:u8,
    score:u8,
}
impl Student {
    fn impl_print(&self){ //& will not moove ownership
        println!("{:#?}",self.name);
    }
}
impl Student {
    fn return_val(self) ->u8 {
        self.score
    }
}
    
fn main(){
    let mut student1 = &Student {
    name:"Waleed".to_string(),
    score:86,
    };

        print(student1);

        fn print (student_1:&Student) {

        println!("{:#?}",student_1.name);
        
    }

let guest = Student {
    name:"Taimoor Imtiaz".to_string(),
    score:100,
};
    print(&mut student1);
    guest.impl_print();

    println!("{:?}",guest);

    let res = guest.return_val();
    println!("Result :{}",res);
}