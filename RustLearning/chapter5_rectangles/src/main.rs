// 结构体demo

// 方法一：分别指定长和宽
// fn main() {
//     let len = 15;
//     let wid = 10;

//     println!("area:{}\n", area(len, wid));
// }

// fn area(length : u32, width : u32) -> u32 {
//     length * width
// }

// 方法二：使用元组打包
// fn main() {
//     let rect = (15, 10);

//     println!("area:{}\n", area(rect));
// }

// fn area(dimensions : (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// 方法三：使用结构体
// #[derive(Debug)]
// struct Rect {
//     length : u32,
//     width : u32,
// }

// fn main() {
//     let rect = &Rect {
//         length : 15,
//         width : 10,
//     };

//     println!("rect is {:#?}\n", rect);

//     println!("area:{}\n", area(rect));
// }

// fn area(rectangle: &Rect) -> u32 {
//     rectangle.length * rectangle.width
// }

// 打印：
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
