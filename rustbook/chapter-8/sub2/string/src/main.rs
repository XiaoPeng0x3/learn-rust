/**
 * 字符串内容
 * 
 */
fn main() {
    // 1. 新建一个字符串
    let mut s = String::new();
    // 可以使用push_str进行添加
    s.push_str("Hello, ");
    // 还可以使用push添加单个字符
    s.push('W');
    println!("s: {}", s);

    // 2. 使用to_string()方法将字符串字面量转换为String
    let s2 = "world".to_string();
    println!("s2: {}", s2);

    // 3. 使用String::from()方法将字符串字面量转换为String
    let s3 = String::from("Hello, Rust!");
    println!("s3: {}", s3);

    // 4. 字符串拼接
    // 使用 + 运算符进行拼接
    let s4 = s + &s2; // 注意这里需要使用引用
    println!("s4: {}", s4);
    // 使用 format! 宏进行拼接
    let s5 = format!("{}---{}", s3, s2); // 中间支持我们自定义格式
    println!("s5: {}", s5);

    // 5. 字符串不支持索引
    // let first_char = s[0]; // 这行代码会报错，因为字符串不支持索引
    // 但是可以使用 chars() 方法获取字符迭代器
    // 不支持索引是因为有一些字符并不是ascii字符，可能占用多个字节
    // 使用索引时会导致不确定性和错误

    // 6. 字符串切片
    let s6 = &&s2[0..5]; // 取前5个字符
    println!("s6: {}", s6);

    // 7. 字符串遍历
    // 有些时候可能真的需要我们去遍历字符串中的字符
    for c in s3.chars() {
        println!("Character: {}", c);
    }
    // byte原始字节
    for b in s3.bytes() {
        println!("Byte: {}", b);
    } 
}
