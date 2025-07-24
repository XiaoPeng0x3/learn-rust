use core::{cmp::PartialOrd, marker::Copy};

/**
 * 使用泛型
 * 
 */
fn main() {
    println!("Hello, world!");
}

// 1. 寻找数组中的最大值
fn get_max_value<T: PartialOrd + Copy> (list: &[T]) -> T {
    let mut larget_num = list[0];
    for &num in list{
        if num > larget_num {
            larget_num = num;
        }
    }
    larget_num
}
