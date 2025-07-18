fn main() {
    // 今天学习的是函数有关的内容
    print_hello();
    say_number(20);
    println!("10 + 20 = {}", add_two_num(10, 20));
    print_num(0);
}

fn print_hello() {
    println!("Hello,World");
}

fn say_number(num: i32) {
    println!("I can say number {}", num);
}

fn add_two_num(a: i32, b: i32) -> i32 {
    return a + b;
}

// 现在来写一个递归函数看看
fn print_num(num: i32) {
    if (num > 10) {
        return;
    }
    println!("num = {num}");
    print_num(num + 1);
}
