#![allow(dead_code)]
#![allow(unused_variables)]

// rust中每个引用都有使自己保持有效的作用域，即生命周期
// 大多数情况生命周期是隐式的，可被推断的,只有在在生命周期互相关联时，需要显示的标注
//生命周期标注描述了多个引用的生命周期之间的关系，但并不影响生命周期
//下面的'a表示a和b两个引用中生命周期较小的那个
//从函数返回引用时，返回的类型的生命周期至少要与其中一个参数的生命周期相匹配
//否则只能返回函数内部创建的引用，也就是返回了一个悬垂引用
fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
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
