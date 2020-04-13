#[derive(Debug)]
struct NewsArticle{
    pub author:String,
    pub content:String,
}
struct Tweet {
    pub username:String,
    pub content :String,

}
pub trait Summary{
    fn summarize(&self)->String;//this is custom behavior if you want to make it default implement it here.
}
impl Summary for NewsArticle{
    fn summarize(&self)->String{
        format!("{} by {}",self.author,self.content)
    }
}
impl Summary for Tweet{
    fn summarize(&self)->String{
        format!("{} Tweeted {}",self.username,self.content)
    }
}
//type is defined

fn main() {
let tweet_1 = Tweet{
    username :String::from("Jhon"),
    content:String::from("Honesty is the best policy"),
};
let new_article_1 = NewsArticle{
    author:String::from("Jeff"),
    content:String::from("Its Raining Cats and Dogs"),
};
println!("{:#?}",tweet_1.summarize() );
println!("{:#?}",new_article_1.summarize() );
}
