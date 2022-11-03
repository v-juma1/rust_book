#![allow(dead_code)]
#![allow(unused_variables)]
use std::collections::HashMap;
#[derive(Debug)]
enum TableCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn vector_example() {
    //创建vector的两种方式
    //创建空的vector
    let v1: Vec<i32> = Vec::new();

    //使用初始值的方式创建vector
    //注意vec宏的v是小写的
    let mut v2 = vec![1, 2, 3];

    //添加元素
    v2.push(4);
    v2.push(5);

    //读取vector元素的两种方式
    let third = &v2[2];
    println!("{}", third);

    match v2.get(2) {
        Some(third) => println!("{}", third),
        None => println!("None"),
    }

    //通过 结构体在vec中存储不同类型的数据
    let v3 = vec![
        TableCell::Int(32),
        TableCell::Float(3.8),
        TableCell::Text(String::from("text content")),
    ];

    println!("{:#?}", v3)
}

fn string_example() {
    let mut s = String::from("hello");

    //更新string
    s.push_str(" everyone");
    s.push('!'); //添加单个字符

    println!("{}", s);
    //字符串拼接
    let s1 = String::from("very ");
    let s2 = "good".to_string();
    //操作符号第二个字符是引用
    let s3 = s1 + &s2;
    println!("{}", s3);

    //或者使用format宏进行拼接
    //不会取得任何参数的所有权
    let s4 = format!("{},{},{}", s, s2, s3);
    println!("{}", s4);

    //string的遍历
    let s5 = "नमस्ते";
    //遍历字节
    for b in s5.bytes() {
        println!("{}", b);
    }

    //遍历标量值
    for c in s5.chars() {
        println!("{}", c);
    }

    //遍历标量值
}

fn hashmap_example() {
    let mut m1: HashMap<String, i32> = HashMap::new();

    m1.insert(String::from("math"), 10);
    m1.insert(String::from("english"), 8);

    //使用collect方法创建hashmap
    let v1 = vec!["math", "english"];
    let v2 = vec![1, 2, 3];

    let m2: HashMap<_, _> = v1.iter().zip(v2.iter()).collect();

    //使用变量的引用不发生所有权的移动
    let mut m3 = HashMap::new();
    let k1 = String::from("k1");
    let v1 = 7;
    m3.insert(&k1, v1);

    //访问hashmap中的值
    let value = m3.get(&k1);
    match value {
        Some(v) => println!("{}", v),
        None => (),
    }

    //更新hashmap
    //插入重复的key，value会被新值覆盖
    m1.insert(String::from("math"), 8);

    //使用entry方法，只有在key不存在时插入
    m1.entry(String::from("chinese")).or_insert(6);

    //遍历hashmap
    for (k, v) in &m1 {
        println!("{}:{}", k, v);
    }
}
fn main() {
    //vector_example();
    //string_example();
    hashmap_example();
}
