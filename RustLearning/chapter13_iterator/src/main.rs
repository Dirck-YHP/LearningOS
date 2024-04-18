fn main() {
    let v1 = vec![3, 5, 7];

    for val in v1.iter() {
        println!("got: {}", val);
    }

    let v2: Vec<i32> = vec![1, 4, 6];
    let v3: Vec<_> = v2.iter().map(|x| x + 1).collect();

    assert_eq!(v3, vec![2, 6, 7]);
}
