/**
 * read_line
 * 等待输入一句话，然后原封不动的返回
 * 就相当于scanf一个字符串，然后输出就行
 * 
 */
use std::io;
 fn main() {
    // 创建一个字符串对象input
    let mut input = String::new();
    // 从输入中读取
    io::stdin().read_line(&mut input).expect("读取失败！");
    println!("{}", input);
}
