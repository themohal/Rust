#[derive(Debug)]

struct ImportantExcerpt<'a>{ //ImportantExcerpt meaning detail of anything that is important. 'a is lifetime 
    part:&'a str,     //'a tells compiler keep the data in the datafield untill the field itself exists
}
fn main() {
     let novel = String::from("Today is friday. and it is very hot outside");
        let new_novel = novel.split(".").next().expect("failed to split '.' ");
        let p1 = ImportantExcerpt{
            part:new_novel,
        };
        println!("{:#?}",p1 );

}
