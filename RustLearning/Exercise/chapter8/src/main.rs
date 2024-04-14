use core::num;
// 给定一系列数字，使用 vector 并返回这个列表的平均数（mean, average）、
// 中位数（排列数组后位于中间的值）和众数（mode，出现次数最多的值；这里哈希 map 会很有帮助）。
use std::collections::HashMap;

fn mean(numbers: &[f64]) -> f64 {
    let sum: f64 = numbers.iter().sum();
    sum / numbers.len() as f64
}

fn median(numbers: &mut [f64]) -> f64 {
    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let len = numbers.len();
    if len % 2 == 0 {
        let mid = len / 2;
        (numbers[mid - 1] + numbers[mid]) / 2.0
    } else {
        numbers[len / 2]
    }
}

fn mode(numbers: &[f64]) -> f64 {
    let mut freq_map: HashMap<&f64, usize> = HashMap::new();

    for &num in numbers {
        let count = freq_map.entry(&num).or_insert(0);
        *count += 1;
    }

    *freq_map.iter().max_by_key(|&(_, count)| count).unwrap().0
}

fn main() {
    let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0, 5.0, 5.0, 6.0, 7.0];
    
    let mean_value = mean(&numbers);
    let mut sorted_numbers = numbers.clone();
    let median_value = median(&mut sorted_numbers);
    let mode_value = mode(&numbers);
    
    println!("Mean: {}", mean_value);
    println!("Median: {}", median_value);
    println!("Mode: {}", mode_value);
}

