fn main() {
    let s = String::from("sth");
    takes_ownership(s);
    // println!("{}", s);

    let x = 88;
    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
