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
    format!("@{} Posted This: {}",self.author,self.content)
}
}
//trait bound syntax
    fn notify<T:Summary>(item:T)->String{ //<T:Summary> this tells that T is some generic type which implemnts summary trait
        format!("{}",item.summarize())
        }
        
fn main() {

    let tweet_1 = Tweet{ //any instance should have impl for trait for trait bound syntax
        username:String::from("Jhon"),
        content:String::from("Hello World"),        
    };
    let article_1 = NewsArticle{
        author:String::from("David"),
        content:String::from("This is first article"),
    };
    println!("{:#?}",tweet_1.summarize() );
    println!("{:#?}",article_1.summarize() );
    //now by trait bound syntax we just pass the instance
    println!("{:#?}",notify(tweet_1) );
}
