// enum IpAddrKind {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// fn main() {
//     // let ipv4 = IpAddrKind::V4;
//     // let ipv6 = IpAddrKind::V6;

//     let home = IpAddrKind::V4(127, 0, 0, 1);
//     let loopback = IpAddrKind::V6(String::from("::1"));

//     println!("Hello, world!");
// }

/*--------------------------------------------------- */
fn main() {
    let some_value: Option<i32> = Some(5); // 创建一个包含整数值的 Some
    let none_value: Option<i32> = None;    // 创建一个空值 None

    match some_value {
        Some(value) => println!("The value is: {}", value),
        None => println!("There is no value."),
    }

    match none_value {
        Some(value) => println!("The value is: {}", value),
        None => println!("There is no value."),
    }
}
