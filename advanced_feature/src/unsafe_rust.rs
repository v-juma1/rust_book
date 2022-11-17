#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
//使用static声明全局静态变量，拥有'static生命周期
static A: &str = "hello";

//也可以声明可变的静态变量
static mut GLOBAL_NUM: i32 = 10;

pub fn main() {
    println!("全局变量：{}", A);

    //访问和修改可变的静态变量也是unsafe的
    unsafe {
        GLOBAL_NUM += 1;
        println!("修改全局变量：{}", GLOBAL_NUM)
    }

    raw_pointer();

    //unsafe函数只能在unsafe代码块中调用
    unsafe {
        unsafe_raw_pointer();
    }

    //extern 函数只能在unsafe代码块中调用
    //需要配置好ABI环境，否则报错
    // unsafe {
    //     add(2, 3);
    // }
}

//原始指针的实例
//原始指针可以在safe代码块中定义，但解引用只能在unsafe代码块中
fn raw_pointer() {
    let mut num = 5;

    //定义一个可变的原始指针
    let r1 = &mut num as *mut i32;
    //定义一个不可变的原始指针
    let r2 = &num as *const i32;
    unsafe {
        println!("{}", *r1);
        println!("{}", *r2);
    }
}

//或者使用unsafe 关键字声明unsafe函数
//在unsafeh函数中进行不安全的操作，不需要额外声明unsafe代码块
unsafe fn unsafe_raw_pointer() {
    let mut num = 100;

    //定义一个可变的原始指针
    let r1 = &mut num as *mut i32;
    //定义一个不可变的原始指针
    let r2 = &num as *const i32;

    println!("{}", *r1);
    println!("{}", *r2);
}

//使用extern函数调用外部代码
extern "C" {
    fn add(a: i32, b: i32) -> i32;

}

//使用extern关键字将函数导出为其他语言可用的接口
//no_mangle 可以避免rust在编译过程中改变函数的名称
#[no_mangle]
pub extern "C" fn say_hi() {
    println!("hello!")
}

//声明unsafe trait
unsafe trait Demo {}

//在unsafe代码块中实现unsafe trait
unsafe impl Demo for String {}
