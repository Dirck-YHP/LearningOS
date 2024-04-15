// fn largest(list: &[i32]) ->i32 {
//     let mut largest = list[0];

//     for &num in list {
//         if num > largest {
//             largest = num;
//         }
//     }

//     largest
// }

// fn main() {
//     let num_list = vec![34, 50, 25, 100, 65];
//     let result = largest(&num_list);
//     println!("The largest num is {}", result);

//     let num_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
//     let result = largest(&num_list);
//     println!("The largest num is {}", result);
// }


// fn largest_i32(list: &[i32]) -> i32 {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn largest_char(list: &[char]) -> char {
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

//     let result = largest_i32(&number_list);
//     println!("The largest number is {}", result);
//    assert_eq!(result, 100);

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest_char(&char_list);
//     println!("The largest char is {}", result);
//    assert_eq!(result, 'y');
// }

// fn largest<T>(list: &[T]) -> T {
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

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest(&char_list);
//     println!("The largest char is {}", result);
// }



#![allow(unused)]
fn main() {
    struct Point<T> {
        x: T,
        y: T,
    }

    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let p1 = Point {x: 5, y: 6};
    let p2 = Point {x: 3.0, y: 4.0};

    let p3 = Point::distance_from_origin(&p2);
    println!("{:?}", p3);

    let p4 = p2.distance_from_origin();

    let p5 = p2.distance_from_origin();
}

