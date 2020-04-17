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
impl Summary for &Tweet {
fn summarize(&self)->String{
    format!("@{} Posted This: {}",self.username,self.content)
}
}

impl Summary for &NewsArticle{
fn summarize(&self)->String{
    format!("@{} Posted This: {}",self.author,self.content)
}
}
//trait bound syntax 
    fn notify<T>(item:T)->String //<T:Summary> this tells that T is some generic type which implemnts summary trait
        where T:Summary, //where tells that T is implementing summary we add more traits like Summary+Display
        {
        format!("{}",item.summarize())
        }
        
        //above function can be written as below as long as they both have same impl
    //also called impl trait syntax
     
       //by the function definition we can have different items but 2nd item is bound to have the type of first T type and first for 2nd
        //trait bound syntax
        fn notify3<T,U>(item1:T,item2:U)->String
        where T:Summary,
                U:Summary,
        { //error when passing
            format!("{}{}",item1.summarize(),item2.summarize())
    }
fn main() {

    let tweet_1 = Tweet{ //any instance should have impl for trait for trait bound syntax
        username:String::from("Jhon"),
        content:String::from("Hello World"),        
    };
    let tweet_2 = Tweet{ //any instance should have impl for trait for trait bound syntax
        username:String::from("Mike"),
        content:String::from("Hello Not World"),        
    };
    let article_1 = NewsArticle{
        author:String::from("David"),
        content:String::from("This is first article"),
    };
   // println!("{:#?}",tweet_1.summarize() );
   // println!("{:#?}",article_1.summarize() );
    //now by trait bound syntax we just pass the instance
    println!("{:?}",notify(&tweet_1) );//to pass antoher type we will need create another tweet instance
    println!("{:?}",notify3(&tweet_1,&article_1) );

}
