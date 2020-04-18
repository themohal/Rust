#[derive(Debug)]
struct ImportantExcerpt<'a>{
    part:&'a str,
}

impl <'a> ImportantExcerpt <'a>{
    fn new(&self)->i32{
        3
    }
    fn announce_return_part(&self,announcement:&str)->&str{
        println!("{}",announcement ); //rule 3 lifetime elision
        self.part //returning struct value
    }
}
fn main() {
    let i = ImportantExcerpt{
        part:"Hello World",
    };
    println!("{}", i.new());
    println!("{}",i.announce_return_part("This is running"));

}
