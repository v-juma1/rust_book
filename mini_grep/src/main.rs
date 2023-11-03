use mini_grep;
use std::{env, process};
fn main() {
    let config = mini_grep::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("解析参数发生错误：{}", err);
        process::exit(1);
    });

    if let Err(e) = mini_grep::run(config) {
        eprintln!("程序发生错误：{}", e);
        process::exit(1);
    };
}

//cargo run --bin mini_grep co 'test.txt' case_sensitive
