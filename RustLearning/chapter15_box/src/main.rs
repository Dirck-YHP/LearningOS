// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// use crate::List::{Cons, Nil};

// fn main() {
//     let list = Cons(1, 
//                 Box::new(Cons(2, 
//                     Box::new(Cons(3, 
//                         Box::new(Nil))))));   
// }


// fn main() {
//     let x = 5;
//     let y = Box::new(x);

//     assert_eq!(5, x);
//     assert_eq!(5, *y);
// }


// use std::ops::Deref;
// struct MyBox<T>(T);

// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }

// impl<T> Deref for MyBox<T> {
//     type Target = T;
    
//     fn deref(&self) -> &T {
//         &self.0
//     }
// }

// fn main() {
//     let x = 5;
//     let y = MyBox::new(x);

//     assert_eq!(5, x);
//     assert_eq!(5, *y);
// }



struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer { data: String::from("my value")};
    let d = CustomSmartPointer { data: String::from("other value")};
    println!("over");

    drop(c);
}