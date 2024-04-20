fn main() {
    let mut num = 10;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("Value via r1: {}", *r1);
        *r2 = 20;
        println!("Value via r2: {}", *r2);
    }
}