fn main() {
    // 1. 字符串切片
    let s1 = String::from("Hello, World!");
    // 获取[1,3]范围的字符
    let s2 = &s1[1..3]; // 左闭右开区间
    println!("原始字符串s1 = {s1}");
    println!("s1的切片字符串s2 = {s2}");
    // 获取全部
    let s3 = &s1[..];
    println!("s1的切片字符串s3 = {s3}");
    // 获取2~结尾
    let s4 = &s1[2..];
    println!("s1的切片字符串s4 = {s4}");
    // 获取开头到5
    let s5 = &s1[..5];
    println!("s1的切片字符串s5 = {s5}");

    // 2. 数组也有切片
    let arr1 = [1,2,3,4,5];
    // 数组切片
    let arr2 = &arr1[1..3];
    // 3. 字符串直接切片
    let s5 = "hello";
    let s6 = &s5[1..2];
}
