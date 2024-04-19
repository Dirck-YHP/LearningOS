use std::cell::RefCell;

fn main() {
    let cell = RefCell::new(10);

    // 不可变借用
    {
        let borrowed = cell.borrow();
        println!("The value is: {}", borrowed);
    } // borrowed的借用在这里结束

    // 可变借用
    {
        let mut borrowed_mut = cell.borrow_mut();
        *borrowed_mut += 10;
        println!("The value has been changed to: {}", borrowed_mut);
    } // borrowed_mut的借用在这里结束
}