/**
 * 看看怎么使用vector
 * 
 */

fn main() {
    // 新建一个vector
    let mut v: Vec<i32> = Vec::new();

    // 使用宏新建一个vector
    let v2 = vec![1, 2, 3];

    // 注意所有权机制
    // 存入一些元素
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);

    // 访问元素
    println!("v[0] = {}", v[0]);

    // 迭代访问
    for x in &v {
        println!("x = {}", x);
    }

    println!("v2[0] = {}", v[0]);

    // 2. 访问越界索引
    let x = v.get(100); // 这里会返回一个Option类型, 返回的是一个null值
    // 访问越界的索引会导致panic, 所以使用get方法来避免panic
    // 索引越界，则会导致panic
    match x {
        Some(val) => println!("v[100] = {}", val),
        None => println!("v[100] 越界了"),
    }

    // 3. 获取有效引用后不能再修改元素
    // 这是因为如果原来的空间内存不足以存储新的元素，Rust会移动原来的元素到新的空间，
    // 这会导致原来的引用失效。
    // 下面的代码会导致编译错误
    let x = &v[0];
    // v.push(5); // 这里会导致编译错误，因为x是一个不可变引用 
    println!("x = {}", x);
}
