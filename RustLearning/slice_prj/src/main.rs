// fn main() {
//     let s = String::from("shabi a  hhhhh");

//     let len = first_word(&s);

//     println!("len {}", len);
// }

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

fn main() {
    let s = String::from("hahah shabi hhh ");

    let len = first_word(&s);

    println!("len {}", len);
}

fn first_word(s : &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
