fn main() {
    // 可以变量遮盖， 不可以重新赋值
    let x = 5;
    println!("The value of x is: {}", x);
    let x = 6;
    // x = 10; 错误！！！！ 
    println!("The value of x is: {}", x);

    // 常量永远不变，声明时需要指定类型
    const DAY_HOURS: u32 = 24;
    println!("一天有{}小时", DAY_HOURS);
}
