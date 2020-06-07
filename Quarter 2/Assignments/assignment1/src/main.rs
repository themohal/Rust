use std::fmt::Display;
#[derive(Debug)]
struct schoolReport<T:Display,U:Display,V:Display>{
    studentName:T,
    studentClass:U,
    studentGrade:V,
}
impl <T:Display,U:Display,V:Display> schoolReport<T,U,V> {
    fn displayReport(&self)->String{
        format!("Name: {} Class: {} Grade: {}",self.studentName,self.studentClass,self.studentGrade)
    }
}
fn main(){
    let nGrade = schoolReport{studentName:"Arslan",studentClass:"5th",studentGrade:5.2};
    let aGrade = schoolReport{studentName:"Arslan",studentClass:"5th",studentGrade:"A+"};
    println!("Numeric Grade Report: {}",nGrade.displayReport());
    println!("Alphabetic Grade Report: {}",aGrade.displayReport());    
}
