#[derive(Debug)]
struct Tweet {
    username:String,
    content:String,
}
#[derive(Debug)] //why we need this again
struct NewsArticle{
    author:String,
    content:String,
}
pub trait Summary{
    fn summarize(&self)->String;
}
impl Summary for Tweet {
fn summarize(&self)->String{
    format!("@{} Posted This: {}",self.username,self.content)
}
}
impl Summary for NewsArticle{
fn summarize(&self)->String{
    format!("@{} Wrote {}",self.author,self.content)
}
}
fn create_summary1()-> impl Summary{ //we dont need to tell data type explicitly compiler will know whatever datatype is passed it will have summary trait implemented
        Tweet{
            username:String::from("Jhon"),
            content:String::from("Hello World"), 
        }
}
//NOTE:We can only have 1 datatype at time to create instance we cannot make 2 instances of different datatype
 fn create_summary2()-> impl Summary{
    NewsArticle{
        author:String::from("David"),
        content:String::from("This is first article"),
    }
 }       
fn main() {

    let tweet_1 = create_summary1();
    let article_1=create_summary2();
    
    println!("{:#?}",tweet_1.summarize() );
    println!("{:#?}",article_1.summarize() );
    
}

