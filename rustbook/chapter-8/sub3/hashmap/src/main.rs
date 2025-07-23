use std::collections::HashMap;

/**
 * 看看`Rust`怎么使用哈希map
 */
fn main() {
    // 1. 新建一个HashMap
    let mut map = HashMap::new();
    map.insert(String::from("111"), 10);
    map.insert(String::from("222"), 20);
    // 还可以使用zip进行绑定
    let _teams  = vec![String::from("Blue"), String::from("Yellow")];
    let _initial_scores = vec![10, 50];
    //let mut scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // 2. 访问HashMap的元素
    // 根据key获取value
    if let Some(score) = map.get(&String::from("111")) {
        println!("Score for 111: {}", score);
    } else {
        println!("No score found for 111");
    }

    // 3. 遍历HashMap
    for (key, value) in &map {
        println!("Key: {}, Value: {}", key, value);
    }

    // 4. 更新HashMap中的值
    // 如果key存在则更新值
    if let Some(score) = map.get_mut(&String::from("111")) { // 返回的是引用类型
        *score += 5; // 增加5分
    } else {
        println!("No score found for 111 to update");
    }

    // 直接覆盖
    map.insert(String::from("111"), 30); // 覆盖之前的值

    // 只有对应的key不存在时才插入
    map.entry(String::from("333")).or_insert(100);


}
