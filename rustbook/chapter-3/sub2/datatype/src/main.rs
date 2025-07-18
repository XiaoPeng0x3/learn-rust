fn main() {
    // 元组的声明
    let tup = (1, 2, 3);
    let tup: (i32, i16, f32) = (1, 2, 3.0);

    // 元组的解构
    let (x, y, z) = tup;
    println!("解构：{}", x);

    println!(".形式 {}", tup.1);

    // 数组
    let arr = [1,2,3,4];
    // 当确定好数组内的元素类型和个数时，就可以显示的分配一定的空间
    let arr2 = [3;5]; // 分配5个元素，每个元素初始化为3
    const A:usize = 10;
    let arr3 = [0; A];
}
