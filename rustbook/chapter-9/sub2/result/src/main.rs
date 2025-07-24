use core::result::Result::{self, Err, Ok};
use std::{fs::File, io::{self, ErrorKind}};
use std::io::Read;

fn main() {
    // 1. 错误处理
    // let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(value) => value,
    //     Err(error) => {
    //         panic!("Can't open file {:?}", error)
    //     },
    // };

    // 2. 错误的类型
    // 我们可以改写一下上面的代码， 如果是找不到该文件，那么就创建该文件
    let f = File::open("Hello.txt");
    let f = match f {
        Ok(val) => val,
        // 打开失败， 看看是什么类型的错误
        Err(error) => match error.kind() {
            // 找不到，那么就新建一个文件
            ErrorKind::NotFound => match File::create("Hello.txt") {
                // 新建文件也有可能会失败
                Ok(fc) => fc,
                Err(err) => panic!("Problem creating the file: {:?}", err),
            }
            other_error => panic!("Problem opening the file: {:?}", other_error)
        },
    };

    // 4. main函数也可以返回Result
    // use std::error::Error;
    // use std::fs::File;

    // fn main() -> Result<(), Box<dyn Error>> {
    //     let f = File::open("hello.txt")?;

    //     Ok(())
    // }

}

// 3. 传播错误
fn read_username_from_file() -> Result<String, io::Error> {
    // 打开一个文件
    // let f = File::open("hello.txt");
    // let mut f = match f {
    //     Ok(val) => val,
    //     Err(err) => return Err(err),
    // };
    // let mut s = String::new();

    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }

    // 逻辑简化 ?
    let mut f = File::open("Hello.txt")?; // 自动返回失败的情况
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    // return Ok(s);
    // 作为return的时候不要加分号
    Ok(s)

}
