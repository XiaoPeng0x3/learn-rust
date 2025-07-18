#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 3. 定义一个方法
impl Rectangle {
    // 以下两种方法等价
    fn area(&self) -> u32{
        return self.width * self.height;
    }
    fn width(self: &Self) ->u32 {
        return self.width;
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }

    // 实例方法
    fn Square(size: u32) -> Rectangle {
        return Rectangle {
            width: size,
            height: size,
        };
    }
}


fn main() {
    // 今天来看看怎么实现打印各种各样的数据
    // 使用println!宏有时候不能打印我们自己的数据，这里我感觉是一个缺点吧。
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    // 可以看到，下面的代码是有问题的，编译器提示我们要选择对应的格式
    // println!("{}", rect1);

    // 然后又提示我们要使用#[driver(Debug)]
    //println!("{:?}", rect1);

    println!("{:?}", rect1);
    /*
        对应输出
        Rectangle { width: 30, height: 50 }
     */

    // 还可以使用 {:#?}来进行使用
    println!("{:#?}", rect1);
    /*
        Rectangle {
            width: 30,
            height: 50,
        }
     */

    // 2. dbg!宏
    // An example:
    // let a = 2;
    // let b = dbg!(a * 2) + 1;
    //      ^-- prints: [src/main.rs:2:9] a * 2 = 4

    // dbg!宏是我们调试的一个好帮手，它会计算出括号内的内容，并将值返回给所有者
    let a = String::from("abc");
    dbg!(&a);
    println!("{a}");

    // 3. 给这个类结构体定义一些方法
    let area = rect1.area();
    println!("{:?}面积是: {area}", rect1);

    // 4. 更多函数参数
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // 5. 构造方法
    // 也可以理解为返回实例的方法，不需要加&self参数
    // 可以通过::方式获得一个实例，这种方法叫做关联方法

    let rec4 = Rectangle::Square(100);
    println!("{:?}", rec4);

}
