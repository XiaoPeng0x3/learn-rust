// 创建一个user结构体
struct User {
    name: String,
    age: u16,
    email: String,
}

fn main() {
    // 1. 使用结构体
    let mut user1 = User {
        name: String::from("zxp"),
        age: 18,
        email: String::from("zxp110@xyz.com"),
    };

    // 2. 改变User的某个属性,
    user1.age = 20;

    // 3. 快速拷贝
    let user2 = User {
        age: 70,
        // 剩余内容快速复制
        ..user1
    };
    println!("user2的age = {}", user2.age);

    // 注意，上面的user2引用了use1的名字和email，所以发生了权限交换
    // println!("user1的名字 = {}", user1.name);

    // 3.对于字符串类型
    // &str是引用，不发生所有权转移；String是自身拥有权，会发生权限转移
}
