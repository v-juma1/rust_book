#![allow(dead_code)]
#![allow(unused_variables)]
fn option_value() {
    //Option<T> 枚举的两个值Some(T)和None
    let x = Some(5);
    let y = Some(String::from("hello"));
    let z: Option<i32> = None;

    let r1 = option_fun(x);
    let r2 = option_fun(z);
}

//用match 处理option枚举
fn option_fun(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

#[derive(Debug)]
enum Season {
    //枚举绑定值
    Spring(String),
    Summer(String),
    Autumn(String),
    Winter(String),
}

fn month_of_season(s: &Season) {
    match s {
        //match 分支也可以捕获枚举绑定的值
        Season::Spring(x) => println!("the month of Spring is {}", x),
        Season::Summer(x) => println!("the month of Summer is {}", x),
        //下划线通配符，代表其他所有可能的情况
        _ => println!("the Season is not Spring"),
    }
}

//只关心一种匹配，而忽略其他匹配的情况也可以用if let来处理
fn month_of_season_2(s: &Season) {
    //if let 匹配的模式 = 匹配的变量 {匹配的操作} else {其他操作} , else {其他操作}部分可以省略
    if let Season::Spring(_) = s {
        println!("the Season is Spring");
    } else {
        println!("the Season is not Spring");
    }
}

pub fn run_demo() {
    option_value();
    let s = Season::Spring(String::from("1,2,3"));
    month_of_season(&s);
    month_of_season_2(&s);
}
