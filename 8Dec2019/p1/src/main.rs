// // #[derive(Debug)]
// // pub struct Tweet {
// //     username:String,
// //     content: String,
// // }

// // #[derive(Debug)]
// // pub struct NewsArticle {
// //     author:String,
// //     content:String,
// // }

// // impl Tweet{
// //     fn new (username:String, content:String) -> Self {
// //         Self {
// //             username,
// //             content
// //         }
// //     }
// // }

// // impl NewsArticle{
// //     fn new (author:String, content:String) -> Self {
// //         Self {
// //             author,
// //             content
// //         }
// //     }
// // }
// // pub trait Summary {
// //     fn summarize(&self) -> String;
// // }
// // impl Summary for Tweet {
// //     fn summarize(&self) -> String {
// //     format!("@{} Tweeted {}",self.username,self.content)
// //     } 
// // }
// // impl Summary for NewsArticle {
// //     fn summarize(&self) -> String {
// //     format!("@{} Wrote {}",self.author,self.content)
// //     } 
// // }

// // fn main() {
// //     let tweet_1 = Tweet::new("Jhon".to_string(), "Its cool outside".to_string());
// //     let news_article = NewsArticle::new("Jordan B. Peterson".to_string(), "12 Rules for Life".to_string());
// //     println!("{:#?} \n{:#?}",tweet_1.summarize(),news_article.summarize());
// // }

// #[derive(Debug)]
// pub struct Tweet {
//     username:String,
//     content: String,
// }

// #[derive(Debug)]
// pub struct NewsArticle {
//     author:String,
//     content:String,
// }

// impl Tweet{
//     fn new (username:String, content:String) -> Self {
//         Self {
//             username,
//             content
//         }
//     }
// }

// impl NewsArticle{
//     fn new (author:String, content:String) -> Self {
//         Self {
//             author,
//             content
//         }
//     }
// }
// pub trait Summary {
//     fn summarize(&self) -> String {
//         format!("This comes from default implementation")
//     }
// }
// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//     format!("@{} Tweeted {}",self.username,self.content)
//     } 
// }
// impl Summary for NewsArticle {}

// fn main() {
//     let tweet_1 = Tweet::new("Jhon".to_string(), "Its cool outside".to_string());
//     let news_article = NewsArticle::new("Jordan B. Peterson".to_string(), "12 Rules for Life".to_string());
//     println!("{:#?} \n{:#?}",tweet_1.summarize(),news_article.summarize());
// }




// #[derive(Debug)]
// pub struct Tweet {
//     username:String,
//     content: String,
// }

// #[derive(Debug)]
// pub struct NewsArticle {
//     author:String,
//     content:String,
// }

// impl Tweet{
//     fn new (username:String, content:String) -> Self {
//         Self {
//             username,
//             content
//         }
//     }
// }

// impl NewsArticle{
//     fn new (author:String, content:String) -> Self {
//         Self {
//             author,
//             content
//         }
//     }
// }
// pub trait Summary {
//     fn summarize(&self) -> String;


//     fn summarize(&self) -> String {
//                 format!("This comes from default implementation: {}", self.summarize())
//     }
// }

// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         format!("{}",self.username)
//     }
//  }
// impl Summary for NewsArticle {

//     fn summarize(&self) -> String {
//         format!("{}",self.author)
//     }
// }

// fn main() {
//     let tweet_1 = Tweet::new("Jhon".to_string(), "Its cool outside".to_string());
//     let news_article = NewsArticle::new("Jordan B. Peterson".to_string(), "12 Rules for Life".to_string());
//     println!("{:#?} \n{:#?}",tweet_1.summarize(),news_article.summarize());
// }




// #[derive(Debug)]
// pub struct Tweet {
//     username:String,
//     content: String,
// }

// #[derive(Debug)]
// pub struct NewsArticle {
//     author:String,
//     content:String,
// }

// impl Tweet{
//     fn new (username:String, content:String) -> Self {
//         Self {
//             username,
//             content
//         }
//     }
// }

// impl NewsArticle{
//     fn new (author:String, content:String) -> Self {
//         Self {
//             author,
//             content
//         }
//     }
// }
// pub trait Summary {
//     fn summarize(&self) -> String;


    
// }

// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         format!("@{} Teewted {}",self.username,self.content)
//     }
//  }
// impl Summary for NewsArticle {

//     fn summarize(&self) -> String {
//         format!("@{} Wrote {}",self.author,self.content)
//      }
// }

// // fn notify(item:  impl Summary, item_1: impl Summary) {
// //     println!("{}\n{}",item.summarize(), item_1.summarize());
// // }

// fn notify<T: Summary> (item: T, item_1:T) {
//      println!("{}\n{}",item.summarize(), item_1.summarize());
// }

// fn main() {
//     let tweet_1 = Tweet::new("Jhon".to_string(), "Its cool outside".to_string());
//     let tweet_2 = Tweet::new("Jeff".to_string(), "NOOOO".to_string());
    
//     let news_article = NewsArticle::new("Jordan B. Peterson".to_string(), "12 Rules for Life".to_string());
//     notify(tweet_1,tweet_2);
//     //notify(news_article);
// }


// fn main () {
// println!("{}",largest_item(&vec![1,9,4,5]));
// println!("{}",largest_item(&vec!['a','v','l']));

// }


// fn largest_item<T: PartialOrd + Copy>(item:&[T]) -> T {
//     let mut largest = item[0];
//     for &number in item.iter() {
//         if number > largest {
//             largest = number;
//         }
//     }
//     largest
// }






// #[derive(Debug)]
// pub struct Tweet {
//     username:String,
//     content: String,
// }

// #[derive(Debug)]
// pub struct NewsArticle {
//     author:String,
//     content:String,
// }

// pub trait Summary {
//     fn summarize(&self) -> String;
// }

// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         format!("@{} Teewted {}",self.username,self.content)
//     }
// }
// impl Summary for NewsArticle {

//     fn summarize(&self) -> String {
//         format!("@{} Wrote {}",self.author,self.content)
//      }
// }

// fn summarizebale () -> impl Summary {
//     // if x == true {
//     Tweet {
//         username: String::from("Jeff"),
//         content: String::from("It's cold outside!"),
//     }
// //}
// // else {
// //     NewsArticle {
// //         author: String::from("Jordan"),
// //         content:String::from("Its 7 December"),
// //     }
// // }

// }

// fn main() {

//  let return_val = summarizebale();
//  println!("{}",return_val.summarize());

// }
// use std::fmt::Display;
// #[derive(Debug)]
// struct Point <T> {
//     x: T,
//     y: T,
// }

// impl <T> Point <T> {
    
//     fn new (x: T, y: T) -> Self {
//         Self {
//             x,
//             y,
//         }
//     }

// }
// impl <T: Display + PartialOrd> Point <T> {
//     fn cmp_display(&self)  {
//         if self.x > self.y {
//             println!("x is greater than y")
//         }
//         else {
//             println!("y is greater than x")
//         }
//     }
// }
// use std::collections::HashMap;

// fn main () {

//     let point_1  = Point::new(7,8);
//     //println!("{:#?}",point_1);
//     //point_1.cmp_display();

//     let mut h1 = HashMap::new();
//     h1.insert("BLue", 10);
//     let mut h2 = HashMap::new();
//     h2.insert("Green",11);

//     let merge = Point::new(h1,h2);
//     println!("{:#?}",merge);
//     //merge.cmp_display();
// }






// fn main () {
//     let r; 

//     {
//         let x = 10;
//         r = &x;
//     }

//     println!("{}",r);
// }




// fn main () {
//     let mut x =10;
//     let ref1 = &x;
//     println!("{}",ref1);  //reading the memory at x
//     let ref2 = &mut x; //writing to the memory at x
//     println!("{}",ref2);

// }

// Now we will be making a function to find longest of two Strings.


// fn main () {
//     let string1 = String::from("Hello World");
//     let result;
//     {
//     let string2 = String::from("Hello Pakistan");
//     result = longest(string1.as_str(), string2.as_str());
//     }
//     println!("{}",result);
// }


// fn longest<'a> (x: &'a str,y:&'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     }
//     else {
//         y
//     }
// }
//#[derive(Debug)]
// struct importantexcerpt <'a> {
//     part: &'a str,
// }
// impl <'a> importantexcerpt <'a> {
//     fn x (&self) -> i32 {
//         54
//     }
//     fn annoucement (&self, ann: &str) -> &str {
//         println!("{}",ann);
//         self.part
//     }
// }

// fn main () {
//     let line = String::from("Hey. Well! Just being hopefull about he future is not enough...");
//     let words = line.split('.').next().expect("Could not find ."); //Hey
//     //println!("{}",words);
//     let i = importantexcerpt {
//         part: words
//     };
//     println!("{:#?}",i);
//     println!("{}",i.x());
//     println!("{}",i.annoucement("Hello World"));

// }

// fn main () {
//     println!("{}",first_word("Hello World"));
// }

// fn first_word (x: &str) -> &str {
//     let bytes  = x.as_bytes();
//     println!("{:#?}",bytes);
//     for (i,&item) in bytes.iter().enumerate() {
//         if item == 32 {
//            return  &x[0..i]
//         }

//     }
//     &x[..]
// }
// use std::fmt::Display;
// fn main() {
   
// println!("{}",genrics_traits_lifetimes("Hello world", "Hello Pakistan", 6.431));
// }


// fn genrics_traits_lifetimes <'a, T>  (x: &'a str, y: &'a str, ann:T) -> &'a str 
// where T: Display {
//     println!("{}",ann);
//     if x.len() > y.len() {
//         x
//     }
//     else 
//     {
//         y
//     }
    
// }




// fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);
//     println!("The largest number is {}", result);

//     let char_list = vec![String::from("hate"),String::from("delight")];

//     let result = largest(&char_list);
//     println!("The largest char is {}", result);
// }
// fn main() {
//     println!("{}",longest("he", "be"));
// }
// fn longest<'a>(x: &'a str, y: &'a str) -> String {
//     let result = String::from("really long string");
//     result.to_owned()
// }







// #[derive(Debug)]
// struct Tweet {
//     username: String,
//     content: String,
// }

// struct NewsArticle {
//     author: String,
//     content: String,
// } //Type define

// pub trait Summary {
//     fn summarize_author(&self)-> String; //method signature
    
//     fn summarize(&self) -> String {
//     format!("This is from default implementation {}",self.summarize_author())}

// } //Trait


// impl Summary for Tweet {
//     fn summarize_author(&self) -> String {
//         format!("@{}",self.username)
//     }

    
// }

// impl Summary for NewsArticle {
//     fn summarize_author(&self) -> String {
//         format!("@{}",self.author)
//     }
// } // Trait implemented


// // fn notify (item: impl Summary, item1: impl Summary) { //impl trait syntax
// //     println!("{}\n{}",item.summarize(),item1.summarize());
// // }

// // fn notify <T: Summary, U:Summary>(item: T, item1: U) { // trait bound syntax
// //     println!("{}\n{}",item.summarize(),item1.summarize());
// // }

// fn summarizeable() -> impl Summary  {
//     //if x ==true {
//     Tweet {
//         username:String::from("A"),
//         content:String::from("B"),
//     }
// }
// // else {
// //     NewsArticle {
// //         author:String::from("C"),
// //         content:String::from("D"),
// //     }
// // }






// fn main () {
//     let return_value = summarizeable();
//     println!("{}",return_value.summarize());
    
// }


// fn main () {
//     let integer_list = vec! [1,5,9,0]; //list integer
//     let char_list = vec!['h','k','d']; //char list
//     let string_list = vec![String::from("Hello"),String::from("World")];
//     println!("{}",largest_item(&integer_list));
//     println!("{}",largest_item(&char_list));
//     println!("{}",largest_item(&string_list));

// }

// fn largest_item <T: PartialOrd + Copy> (item: &[T]) -> T { //[1,5,9,0]
//     let mut largest = &item[0]; //1
//     for &number in item.iter() {
//         if number > largest {
//             largest = number;
//         }
//     }
//     largest
// }

//let age= 17.5;

// fn main () {
//     let r; 
    
//         let x =5;
//         r=&x; //created a pointer which refrence to x
//      //here x is dropped

//     println!("{}",r);
// } 



//Question Number One(1)
// #[derive(Debug)]
// struct Book{  // data type
//     author: String,
//     name: String,
// }

// pub trait BookInformation{ // trait with signature of info
//     fn info(&self)->String;
// }

// impl BookInformation for Book{
//     fn info(&self) -> String{
//         format!("Author: {}, Name: {}", self.author, self.name)
//     }
// }

// impl Book{ // associated function
//     fn new_block(author:String, name:String)-> Book{
//         Book{
//             author,
//             name
//         }
//     }
// }


// fn main() {
//     let result_of_associated_function = Book::new_block( //associate calling
//         String::from("Graydon Hoare"),
//         String::from("Rust The Book")
//     );
//     let result_of_trait_block = Book{ //trait calling
//         author: String::from("Graydon Hoare"),
//         name: String::from("Rust The Book")
//     };

//     print!("Associate Function Result: {:#?}", result_of_associated_function);
//     print!("Using Trait Block {:#?}", result_of_trait_block);
// }



// #![allow(unused_variables)]

// use std::fmt::Display;
// #[derive(Debug)]
// struct Pair<T> {
//     x: T,
//     y: T,
// }

// impl<T> Pair<T> {
//     fn new(x: T, y: T) -> Self {
//         Self {
//             x,
//             y,
//         }
//     }
// }

// impl<T: Display + PartialOrd> Pair<T> {
//     fn cmp_display(&self) {
//         if self.x >= self.y {
//             println!("The largest member is x = {}", self.x);
//         } else {
//             println!("The largest member is y = {}", self.y);
//         }
//     }
// }

// use std::collections::HashMap;
// fn main () {
//     let s1 = String::from("Hello");
//     let s2 = String::from("Hello World");
    
//     let p3 = Pair::new(s1,s2);
//     println!("{:#?}",p3);
//     p3.cmp_display();



//     let mut h1  =  HashMap::new();
//     h1.insert(10, 10);
//     let mut h2  =  HashMap::new();
//     h2.insert(10, 12);

//     let h3 = Pair::new(h1,h2);
//     println!("{:#?}",h3);
//     //h3.cmp_display();
    
// }














































































































