/**
 * 生命周期这一块
 */

fn main() {
    // 找到两个字符串中较长的那个
    let s1 = String::from("1234");
    {
        let s2 = "xyz";
    }
    
    let res = longest_str(&s1, &s2);
    println!("{} ", res)

}


/*
    生命周期错误
    我们不知道str1和str2谁的声明周期长，手动指定一个生命周期可以减少空指针引用
    生命周期与较短者一致
*/
fn longest_str<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}
