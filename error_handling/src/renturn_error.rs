#![allow(dead_code)]
#![allow(unused_variables)]

use std::io::Error;
use std::{fs::File, io::Read};

//返回Result<String, Error>让调用者处理
pub fn read_data_from_file() -> Result<String, Error> {
    let f = File::open("test.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    //函数的最后一个表达式
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

//使用？运算符简化上面的操作，完成同样的功能，？作用如下
//如果Result的结果为Ok，则Ok的结果作为表达式的值返回，代码继续执行
//如果Result的结果为Err，则将Err作为函数的结果return
//?运算符只能用于返回类型为Result的函数
pub fn read_data_from_file_v2() -> Result<String, Error> {
    //?表示open操作结果如果为Ok，则Ok的值作为表达式的结果进行返回，代码继续执行，
    //如果为Err，则将错误作为函数的结果返回
    let mut f = File::open("test.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
//用链式调用的方法进一步简化上面的函数
pub fn read_data_from_file_v3() -> Result<String, Error> {
    let mut s = String::new();
    File::open("test.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

//标准库提供了函数对上面的方式进行了封装，进一步简化
pub fn read_data_from_file_v4() -> Result<String, Error> {
    std::fs::read_to_string("test.txt")
}
