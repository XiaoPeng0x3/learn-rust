fn main() {
    // 1. 引用不会修改控制权
    let s1 = String::from("hello");
    let s2 = &s1;
    println!("s2 = {}", s2);
    println!("s1 = {s1}");
    // 函数参数的情形
    say_hello(&s1);
    println!("s1 = {s1}");

    // 2.可变才引用可以修改值
    // 尝试去修改s1
    // say_no(s1);
    // 可变引用可以修改
    let mut s3 = String::from("Hello!");
    // 使用引用修改
    say_miao(&mut s3); // 显示声明这个s3是可变的
    println!("s3 = {s3}!");

}
fn say_hello(word: &String) {
    println!("word = {word}");
}

// fn say_no(word: &String) {
//     word.push_str("no");
// }

fn say_miao(word: &mut String) {
    word.push_str(" ,miao!");
}
