fn main() {
    for i in 1..=100 {
        if i % 3 == 0 && i % 5 == 0 { // 即是3的倍数，也是5的倍数
            println!("FizzBuzz!")
        } else if i % 3 == 0 {
            println!("Fizz!")
        } else if i % 5 == 0 {
            println!("Buzz!");
        } else {
            println!("{}", i);
        }
    }
}
