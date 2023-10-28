pub fn run() {
    //只有用mut关键子声明的变量才是可变变量，才能被修改值
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 10;
    println!("The value of x is: {x}");

    //没有mut关键字，即默认声明的是不可变的变量
    //Shadowing：新的同名变量会隐藏之前的同名变量
    let x = 1;
    println!("The value of x is: {x}");

    //声明常量,常量必须声明数据类型
    const MAX_NUM: u32 = 99;
    println!("The value of x is: {MAX_NUM}");

    //声明tuple并使用下标访问
    let t1: (i32, f32, u8) = (-2, 2.5, 1);
    println!("{},{},{}", t1.0, t1.1, t1.2);

    //声明数组并使用索引访问
    let a1: [i32; 3] = [1, 2, 3];
    println!("{},{},{}", a1[0], a1[1], a1[2]);
    //若数组中元素的值都相同，可以这样声明
    let a2 = [1; 3];

    //调用函数
    let r = fun1(a2);
    println!("{}", r);
}

//函数形参、返回值都必须指明类型
fn fun1(x: [i32; 3]) -> i32 {
    x[0] + x[1] + x[2]
}
