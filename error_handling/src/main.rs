#![allow(dead_code)]
#![allow(unused_variables)]
mod handle_error;
mod renturn_error;
fn main() {
    //handle_error::fun1();
    //handle_error::fun2();
    //handle_error::fun3();
    //handle_error::fun4();
    let data = renturn_error::read_data_from_file();
    match data {
        Ok(data) => println!("{}", data),
        Err(e) => panic!("{}", e),
    };
}
