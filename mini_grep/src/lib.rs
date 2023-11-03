use std::{env, error::Error, fs};

#[derive(Debug)]
pub struct Config {
    pub query_string: String,
    pub file_name: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        match args.len() {
            3 | 4 => {
                args.next();

                let query_string = match args.next() {
                    Some(arg) => arg,
                    None => return Err("读取查找的目标字符失败！"),
                };

                let file_name = match args.next() {
                    Some(arg) => arg,
                    None => return Err("读取文件路径失败！"),
                };

                let case_sensitive: bool = env::var("case_sensitive").is_err();

                Ok(Config {
                    query_string,
                    file_name,
                    case_sensitive,
                })
            }
            _ => return Err(
                "参数错误，正确的参数格式为：query file case_sensitive(默认可省略，即不区分大小写)",
            ),
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_name)?;

    if config.case_sensitive {
        for line in search(&config.query_string, &content) {
            println!("{}", line);
        }
    } else {
        for line in search_case_insensitive(&config.query_string, &content) {
            println!("{}", line);
        }
    }

    Ok(())
}
pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

//不区分大小写
pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    //测试search函数
    //cargo test -p mini_grep
    #[test]
    fn test_search() {
        let constent = "\n\
        Common Programming Concepts,\n\
        common function,\n\
        constuctive,";
        let r = search("Co", constent);
        let expc = vec!["Common Programming Concepts,"];
        assert_eq!(expc, r);
    }

    #[test]
    fn test_search_case_insensitive() {
        let constent = "\n\
        Common Programming Concepts,\n\
        common function,\n\
        constuctive,";
        let r = search_case_insensitive("co", constent);
        let expc = vec![
            "Common Programming Concepts,",
            "common function,",
            "constuctive,",
        ];
        assert_eq!(expc, r);
    }
}
