// pub trait Summary {
//     fn summarize(&self) ->String;
// }

// pub struct NewsArticle {
//     pub headline: String,
//     pub location: String,
//     pub author: String,
//     pub content: String,
// }

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

// pub struct Tweet {
//     pub username: String,
//     pub content: String,
//     pub reply: bool,
//     pub retweet: bool,
// }

// impl Summary for Tweet {
//     fn summarize(&self) ->String {
//         format!("{}: {}", self.username, self.content)
//     }
// }

// fn main() {
//     let tweet = Tweet {
//         username: String::from("horse_ebooks"),
//         content: String::from("of course, as you probably already know, people"),
//         reply: false,
//         retweet: false,
//     };

//     println!("1 new tweet: {}", tweet.summarize());
// }

// use std::fmt::{Debug, Display};


// // fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
// //     0
// // }

// fn some_function<T, U>(t: T, u: U) -> i32 
//     where T: Display + Clone,
//           U: Clone + Debug
// {
//     0
// }

// fn main() {
//     let my_string = "Hello".to_string(); // 类型 T 为 String
//     let my_vector = vec![1, 2, 3]; // 类型 U 为 Vec<i32>

//     let result = some_function(my_string, my_vector); // 调用 some_function
// }


fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}