# 说明
这次来看看函数怎么写，函数声明的关键字是`fn`，提到函数，不得不说的就是
- 参数
- 返回值

函数的参数必须显示的声明其类型
```rust
fn say_number(num: i32) {
    println!("I can say number {}", num);
}
```

返回值只需要指明类型即可
```rust
fn add_two_num(a: i32, b: i32) -> i32 {
    return a + b;
}
```

还有一种写法是
```rust
fn add_two_num(a: i32, b: i32) -> i32 {
    a + b
}
```
