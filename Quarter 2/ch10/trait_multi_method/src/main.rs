#[derive(Debug)]
struct NewsArticle{
    pub author:String,
    pub content:String,
}//NewsArticle Type
struct Tweet {
    pub username:String,
    pub content :String,

}//Tweet type
pub trait Summary{
    fn summarize_author(&self)->String;
    fn summarize(&self)->String{
        format!("{}",self.summarize_author())
        //function calling other function it can also have its own implementation
}}
impl Summary for Tweet{
    fn summarize_author(&self)->String{
        format!("{} Tweeted {}",self.username,self.content)
    }
}
impl Summary for NewsArticle {
    fn summarize_author(&self)->String{
        format!("{} by {}",self.author,self.content)
    }
}


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
