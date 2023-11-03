#![allow(dead_code)]
#![allow(unused_variables)]

//下面的'a表示通知借用检查器，这三个引用必须拥有相同的生命周期
//实际指向的是x和y两个引用中生命周期较小的那个，也就是生命周期重叠（公共）的部分
fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//结构体中的引用
//下面的生命周期标注表示struct中的引用存活的时间至少要比结构体的实例存活的时间长
#[derive(Debug)]
struct User<'a> {
    usename: &'a str,
    passwoed: &'a str,
}

pub fn main() {
    let a = "hello";
    let b = String::from("long river");
    let c = longer(a, &b);

    println!("{}", &c);

    let u1 = User {
        //'static生命周期
        //在整个程序运行期间有效
        usename: "test",
        passwoed: "1234",
    };
    println!("{:#?}", u1);

    //引用和User的实例都在main函数中有效
    let usename2 = String::from("user2");
    let passwoed2 = String::from("user2");
    let u2 = User {
        usename: &usename2,
        passwoed: &passwoed2,
    };

    println!("{:#?}", u2);
}
