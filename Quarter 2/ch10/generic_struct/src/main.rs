#[derive(Debug)]
struct Point<T,U,W> {
    x:T,
    y:U,
    s:W
    }
fn main() {
    //creating instances
    //let integer= Point{x:13,y:55};
    //let float = Point{x:2.2,y:4.2};
    //println!("{:#?}",integer );
    //println!("{:#?}",float );
        let integer_float= Point{x:13,y:5.5,s:String::from("OKAY BYE")};
        println!("{:#?}",integer_float ); //we will get error because we only have one generic type to have different type
        //we have to put T and another generic type parameter U in struct



}
