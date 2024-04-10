// 在华氏温度和摄氏度之间转换温度
// use std::io;

// fn main() {
//     let mut ipt = String::new();

//     io::stdin().read_line(&mut ipt).expect("Failed to read line");

//     let ipt: i32 = ipt.trim().parse().expect("Please type a num");

//     let ss = change(ipt);
//     print!("ss: {}", ss);
    
// }

// fn change(huashi : i32) -> i32 {
//     let sheshi = (huashi - 32) * 5 / 9;
//     sheshi
// }


// 生成 n 阶斐波那契数列
use std::io;
fn main() {
    println!("please input n of order fabonaci:");

    let mut n = String::new();

    io::stdin().read_line(&mut n).expect("Failed to read line");

    let n : i32 = n.trim().parse().expect("please type a num");

    print!("n order fabonaci ans: {}", fabonaci(n));
}

fn fabonaci(n: i32) -> i32 {
    if n == 1 {
        1
    }else if n == 2{
        2
    }else {
        fabonaci(n - 1) + fabonaci(n - 2)
    }
}


// 打印圣诞颂歌 “The Twelve Days of Christmas” 的歌词，利用歌曲中的重复部分（编写循环）
// 略