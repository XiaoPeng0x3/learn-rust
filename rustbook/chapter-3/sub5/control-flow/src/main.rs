fn main() {
    // 内部循环打破外部循环
    let mut count = 0;
    'loop_label: loop {
        let mut remain = 10;
        loop {
            // 内部循环终止
            if count == 2 {
                break;
            }
            // 终止外部循环
            if remain == 8 {
                break 'loop_label;
            }
            remain -= 1;
        }
        count += 1;
    }

    // 从循环中返回
    let mut a = 1;
    let b = loop {
        if a > 100 {
            break a;
        }
        a *= 2;
    };
    println!("a > 100时，其值为 {b}");

    // for 循环
    // for .. in
    let arr = [1, 2, 3, 4, 5];
    for ele in arr {
        println!("{ele}");
    }

    // 常规的 for(int i = 0; i < 10; i++)
    for i in 1..10 {
        println!("{i}");
    }

    // if 
    let a = 1;
    let b = 2;
    let c = if a > b {a} else {b};
}
