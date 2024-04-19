// use std::thread;
// use std::sync::mpsc;

// fn main() {
//     let (tx, rx) = mpsc::channel();

//     thread::spawn(move || {
//         let val = String::from("hello");
//         tx.send(val).unwrap();
//     });

//         let received = rx.recv().unwrap();
//         println!("Got: {}", received);
// }

// use std::thread;
// use std::sync::mpsc;
// use std::time::Duration;

// fn main() {
//     let (tx, rx) = mpsc::channel();

//     thread::spawn(move || {
//         let vals = vec![
//             String::from("hi"),
//             String::from("from"),
//             String::from("the"),
//             String::from("thread"),
//         ];

//         for val in vals {
//             tx.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });

//     for received in rx {
//         println!("Got: {}", received);
//     }
// }



use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
// --snip--

let (tx, rx) = mpsc::channel();

let tx1 = tx.clone();
thread::spawn(move || {
    let vals = vec![
        String::from("1 hi"),
        String::from("1 from"),
        String::from("1 the"),
        String::from("1 thread"),
    ];

    for val in vals {
        tx1.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

thread::spawn(move || {
    let vals = vec![
        String::from("2 more"),
        String::from("2 messages"),
        String::from("2 for"),
        String::from("2 you"),
    ];

    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

for received in rx {
    println!("Got: {}", received);
}

// --snip--
}
