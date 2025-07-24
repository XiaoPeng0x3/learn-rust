use core::str;

/**
 * 看看trait
 */

// 有点像接口
pub trait SayHello {
    fn say_hello(&self);
}

pub struct Person {
    name: String,
    age: u32,
    height: f64,
}

pub struct Dog {
    name: String,
    age: u32,
    weight: f64,
}

impl SayHello for Person {
    fn say_hello(&self) {
        let ans = format!("Hello, {}, your age is {} and your height is {}", self.name, self.age, self.height);
        println!("{}", ans);
    }
}

pub trait Speak {
    // 默认实现
    fn speak(&self);
}

// 多态
impl Speak for Person {
    fn speak(&self) {
        println!("I can speak, and I am a human!");
    }
}

// 多态
impl Speak for Dog {
    fn speak(&self) {
        println!("I can speak, and I am a dog!");
    }
}



fn main() {
    // 1. 创建一个实例
    let person: Person = Person { 
        name: String::from("zxp"), 
        age: 20, 
        height: 1.64 
    };
    person.say_hello();
    println!("name = {}", person.name);

    let dog = Dog {
        name: String::from("Puppy"),
        age: 5,
        weight: 32.1,
    };

    // 2. 多态
    who_can_speak(person);
    who_can_speak(dog);

    // 3. 实现多个接口
    // 使用 + 进行拼接
    // pub fn notify(item: impl Summary + Display)
    // pub fn notify<T: Summary + Display>(item: T)
    
}

// 多态
fn who_can_speak(obj: impl Speak) {
    obj.speak();
}

// 有的时候，函数参数可能需要传递很多个Speak类型的参数，像下面这样
// fn who_can_speak(obj1: impl Speak, obj2: impl Speak, obj3: impl Speak)
// 可以改写成下面这样
fn who_can_speak1<T: Speak> (obj: T) {
    obj.speak()
}

