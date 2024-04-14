// fn main() {
//     let v = vec![2, 4, 6, 7, 9];

//     let third: &i32 = &v[3];

//     match v.get(3) {
//         Some(sth) => println!("num is : {}", sth),
//         None => println!("No num found"),
//     }

// }

fn main() {

    let mut s = "initial data".to_string();

    s.push('c');
    s.push_str("hhhh");

    println!("str: {}", s);

    let s1 = String::from("hello");
    let s2 = String::from("world");
    let s3 = String::from("hahaha");

    let s_final = format!("{} {} {}", s1, s2, s3);

    println!("s_final: {}", s_final);

    let part = &s_final[0..1];
    println!("s_part: {}", part);

}


// #![allow(unused)]
// fn main() {
//     for c in "नमस्ते".chars() {
//         println!("{}", c);
//     }

//     for b in "नमस्ते".bytes() {
//         println!("{}", b);
//     }
// }

 