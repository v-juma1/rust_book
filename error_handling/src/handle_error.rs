#![allow(dead_code)]
#![allow(unused_variables)]
use core::panic;
use std::fs::File;
use std::io::ErrorKind;

pub fn fun1() {
    let f = File::open("test.txt");

    //使用match处理Result枚举
    match f {
        Ok(f) => f,
        Err(message) => panic!("打开失败:{}", message),
    };
}

pub fn fun2() {
    //使用unwrap达到与上面代码同样的效果，更简洁
    let f = File::open("test.txt").unwrap();
}

pub fn fun3() {
    //使用expect自定义错误信息
    let f = File::open("test.txt").expect("打开失败（expect）");
    println!("Hello, world!");
}

pub fn fun4() {
    //打开文件，文件不存在则创建
    let f = File::open("test.txt");
    match f {
        Ok(file) => file,

        //再用match分析错误的类型
        Err(error) => match error.kind() {
            //不存在则创建
            //创建文件也有可能失败
            ErrorKind::NotFound => match File::create("test.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("创建失败：{}", e),
            },
            others => panic!("打开失败（不能创建）：{}", others),
        },
    };
}
