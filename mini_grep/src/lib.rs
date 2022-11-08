use std::{error::Error, fs};

#[derive(Debug)]
pub struct Config {
    pub query_string: String,
    pub file_name: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("\
            参数太少：\n\
            参数格式为：query file case_sensitive(默认可省略，即不区分大小写)");
        } else if args.len() == 3 {
            Ok(Config {
                query_string: args[1].clone(),
                file_name: args[2].clone(),
                case_sensitive: false,
            })
        } else if args.len() == 4 {
            if args[3] == "case_sensitive" {
                Ok(Config {
                    query_string: args[1].clone(),
                    file_name: args[2].clone(),
                    case_sensitive: true,
                })
            } else {
                return Err("\
                参数错误：\n\
                参数格式为：query file case_sensitive(默认可省略，即不区分大小写)");
            }
        } else {
            return Err("\
            参数过多：\n\
            参数格式为：query file case_sensitive(默认可省略，即不区分大小写)");
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
    let mut result = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

//不区分大小写
pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let low_query = query.to_lowercase();
    for line in content.lines() {
        if line.to_lowercase().contains(&low_query) {
            result.push(line)
        }
    }
    result
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
