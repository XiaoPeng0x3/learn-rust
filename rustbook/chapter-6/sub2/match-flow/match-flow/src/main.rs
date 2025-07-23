/**
 * 
 * match 控制流
 * if else 只可以匹配bool值，而match可以匹配任意值
 * 来看看Rust是怎么避免产生空指针的
 */

fn main() {
    let a = 10.0;
    let b = 3.0;
    match divide(a, b) {
        Some(val) => println!("{} / {} = {}", a, b, val),
        None => println!("被除数不能为0！")
    }

}

fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        // a / b的结果存储在Some这个枚举成员里面
        Some(a / b)
    }
}
