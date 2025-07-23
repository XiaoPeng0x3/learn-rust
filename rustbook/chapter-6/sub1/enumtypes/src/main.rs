/**
 * 与枚举有关
*/

// 1. 创建一个枚举类型
enum MyIPAddr {
    V4(String),
    V6(String),
}

// 2. 枚举值属性绑定


fn main() {
    // 1. 使用枚举值
    let four = MyIPAddr::V4;
    let six = MyIPAddr::V6;

    // 2. 枚举值属性绑定

}
