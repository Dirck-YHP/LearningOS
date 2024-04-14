use std::collections::HashMap;

// fn main() {
//     let teams = vec![String::from("Blue"), String::from("Red")];
//     let initial_scores = vec![10, 30];

//     let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

//     for (key, value) in &scores {
//         println!("{}: {}", key, value);
//     }

//     let score = scores.get(&"Blue".to_string());
//     println!("score: {:?}", &score);
// }


// fn main() {
//     let mut scores = HashMap::new();
//     scores.insert(String::from("Blue"), 10);

//     scores.entry(String::from("Red")).or_insert(50);
//     scores.entry(String::from("Blue")).or_insert(70);

//     println!("{:?}", scores);
// }


fn main() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    print!("{:?}", map);
}