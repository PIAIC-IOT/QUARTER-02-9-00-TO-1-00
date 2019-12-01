pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
 } // Type defned

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
} //Type defined

pub trait Summary {
    fn summarize(&self) -> String;
} //Trait defined

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
} // Summary trait define on NewsArticle DataType 

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}// Summary trait define on Tweet Datatype




//main fuction
fn main() {
    let my_tweet = Tweet {
        username: String::from("Ali"),
        content: String::from("Winter is coming soon!"),
        reply: false,
    }; // tweet is an instance of type Tweet 
    
    println!("1 new tweet: {}", my_tweet.summarize()); 

    let my_article = NewsArticle {
        headline: String::from("Pakistan Won the Match"),
        location: String::from("Melbourne, Austrailia"),
        author: String::from("Abdul Basit"),
    }; //my_article is an instance of type NewsArticle
    
    println!("\nNew article available! {}", my_article.summarize());  //calling method        
}
