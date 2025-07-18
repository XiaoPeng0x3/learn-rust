# 说明
今天我们来看看控制流，包含`if-else`,`loop`,`for`和`while`

比较新奇的是`loop`循环，新奇点在于一个内部循环也可以将外部循环终止

## 三目运算符？
`if`标签还可以有返回值，有点像`python`
```rust
    // if 
    let a = 1;
    let b = 2;
    let c = if a > b {a} else {b};
```

## loop标签
```rust
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
```
标签使用单引号开头，例如下面一个可以作为一个标签使用
`'label:`

## 从循环中返回
也就是`break`的时候也可以返回一些信息。
```rust
    // 从循环中返回
    let mut a = 1;
    let b = loop {
        if a > 100 {
            break a;
        }
        a *= 2;
    };
    println!("a > 100时，其值为 {b}");

```

## for循环
`while`循环较为简单，不做说明，看看`for`循环怎么写。
### for .. in
可以用于快速遍历数组，也可用于常见的`for (int i = 0; i < 10; i++)
```rust

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

```

