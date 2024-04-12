fn main() {
    println!("Hello, world!");

    let x = another_function(25, 'c');
    println!("x:{}", x);
}

fn another_function(x :i32, unit_label: char) -> i32 {
    println!("The value is {} label {}", x, unit_label);
    x * x
}
