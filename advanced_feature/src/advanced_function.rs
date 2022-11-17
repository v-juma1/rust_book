#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

//将函数作为参数传递给其他函数
fn add(x: i32) -> i32 {
    x + 1
}

fn add_plus(f: fn(i32) -> i32, x: i32) -> i32 {
    f(x) + f(x)
}

//将闭包作为函数的返回值

fn return_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

pub fn main() {
    let res = add_plus(add, 5);
    assert_eq!(12, res);

    let fun = return_closure();
    assert_eq!(6, (*fun)(5));
}
