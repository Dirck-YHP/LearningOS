use std::env;
use std::fs;

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     println!("Hello, world! {:?}", args);

//     let query = &args[1];
//     let filename = &args[2];

//     println!("Searching for {}", query);
//     println!("In file {}", filename);

//     let contents = fs::read_to_string(filename).expect("sth went wrong reading the file");
//     println!("With text:\n{}", contents);
// }

use std::process;
use chapter12_minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

}




