use std::string;

fn main() {
    // 1.所有权
    // 创建一个string类型的变量
    let s1 = String::from("hello");
    // 此时，我们把这个s1内容赋值给s2
    // let s2 = s1;
    // 然后，我们打印一下s1
    println!("字符串内容为: {}", s1);

    // 2.函数转移所有权
    // 调用这个函数
    say_sth(s1);
    // println!("{s1}");报错了！

    // 3.栈上的变量"不受影响"
    let x = 1;
    let y = x;
    println!("x = {x}!");
    println!("y = {y}!");
    // 函数也不会使控制权转移
    add_sth(x);
    println!("x = {x}!");

    // 4. 返回值也会影响所有权
    return_sth(s1);
    println!("字符串内容为: {}", s1);

}

fn say_sth(word: String) {
    println!("I say \"{word}\"!");
}// 到这里, word的所有权已经被转移。

fn add_sth(num: i32) {
    println!("结果是: {}", num + 1);
}

fn return_sth(word: String) -> String {
    // 什么也不做，就是返回自己
    return word;
}