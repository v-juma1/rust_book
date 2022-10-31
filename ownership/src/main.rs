fn main() {
    show_move();

    let s = String::from("hello");

    //s的所有权已经转移给函数move_type，函数move_type调用结束，s销毁
    move_type(s);
    //println!("{}", s); 报错

    let i = 50;

    //基本数据类型实现了copy trait，函数持有的是i变量的拷贝
    refer_type(i);
    //println!("{}", i);可以

    //可以让函数返回参数本身以继续保留参数的所有权
    let s1 = String::from("abcx");
    let (s1, l) = return_owner(s1);
    println!("{},{}", s1, l);

    let (l, s1) = return_owner_2(s1);
    println!("{},{}", l, s1);

    //更简单的方法：借用
    //借用：使用变量的引用作为形式参数
    let length = cal_len(&s1);
    println!("{},{}", length, s1);
}

fn show_move() {
    //基本数据类型是存储在stack上，这样写没问题
    //x和y两个值都被压到stack上
    let x = 5;
    let y = x;
    println!("x:{}", x);
    println!("y:{}", y);

    //复杂类型，数据地址存在stack上，数据存储在heap上
    let s1 = String::from("hello");
    let s2 = s1; //所有权转移，从s1转到s2，s1被销毁
    println!("s2:{}", s2);
    //println!("s1:{}", s1); //报错，s1已经被销毁，引用不存在
}

fn move_type(s: String) {
    println!("{}", s);
}

fn refer_type(i: i32) {
    println!("{}", i);
}

fn return_owner(s: String) -> (String, usize) {
    //返回结果和参数本身以保留参数的生命周期
    let s_len = s.len();
    (s, s_len)
}

//return_owner函数的另一种写法
fn return_owner_2(s: String) -> (usize, String) {
    //len()函数的签名为：pub fn len(&self) -> usize
    //即len函数使用s自身的引用，并不持有所有权
    //但因为最有一个表达式是函数的返回值，所以下面的写法会报错（当然，对应返回值的形参为：(String, usize)）
    //(s, s.len())
    //因为这样写表示s变量已经被本函数即return_owner_2返回，所以s.len()属于“value borrowed here after move” 错误

    (s.len(), s)
}

fn cal_len(s: &String) -> usize {
    s.len()
}
