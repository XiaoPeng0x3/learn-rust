use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // 使用rand包生成一个随机数
    let num = rand::rng().random_range(1..101);
    // 方便调试
    // println!("生成的随机数是{num}！");
    loop {
        let mut input = String::new();
        println!("请输入你猜测的数字： ");
        // 用户读取
        io::stdin()
            .read_line(&mut input)
            .expect("读取失败");
        // 将input转换为u32
        let input: u32 = match input.trim().parse() {
            Ok(input) => input,
            Err(_) => {
                println!("输入的可能不是一个数字哦！"); 
                continue;
            },
        };

        match input.cmp(&num) {
            Ordering::Less => println!("猜小了！"),
            Ordering::Greater => println!("猜大了！"),
            Ordering::Equal => {
                println!("恭喜你猜对了！");
                break;
            },
        }
    }
    
}
