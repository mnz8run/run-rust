// HashMap<K, V>
// 用于需要任何类型作为键来寻找数据的情况，而不是像 vector 那样通过索引

use std::collections::HashMap;

fn main() {
    // 创建
    // 标准库集合部分 导入 HashMap
    // 限制类型相同，键类型彼此相同，值类型彼此相同
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 获取 HashMap 中的值
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{score}");

    // 遍历
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // 更新 覆盖 仅在键不存在时添加键和值 基于旧值更新值

    // 覆盖
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // 仅在键不存在时添加键和值
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(80);
    println!("{:?}", scores);

    // 基于旧值更新值
    let text = "hello world wonderful world";
    let mut repeat_count = HashMap::new();
    for word in text.split_whitespace() {
        let count = repeat_count.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", repeat_count);

    let yellow_score = scores.entry("Yellow".to_string()).or_insert(0);
    *yellow_score += 1;
    println!("{:?}", scores);
}
