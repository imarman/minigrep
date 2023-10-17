use std::{env, fs};
use std::error::Error;

pub fn run(conf: Config) -> Result<(), Box<dyn Error>> {
    // 如果有异常就返回错误
    let contents = fs::read_to_string(conf.file_path)?;

    let results = match conf.ignore_case {
        true => search_w_l_n(&conf.query, &contents),
        false => search_case_insensitive_w_l_n(&conf.query, &contents)
    };

    if results.is_empty() {
        println!("没找到匹配字符")
    }

    for line in results {
        println!("{line}")
    }

    // 没有异常就正常执行, 因为返回值是个 Result 所以需要有一个 Ok
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    /*
    根据命令行的位置区分各个command
     */
    pub fn build(mut args: impl Iterator<Item=String>) -> Result<Config, &'static str> {
        // 第一个参数是程序名，由于无需使用，因此这里直接空调用一次
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        // 有 IGNORE_CASE=1 or true 的情况下会忽略大小写
        let ignore_case = match env::var("IGNORE_CASE") {
            Ok(a) => a == "1" || a == "true",
            Err(_) => false
        };

        Ok(Config { query, file_path, ignore_case })
    }

    /*
    使用 cli 库获取命令行参数
     */
    pub fn build_with_cli(keyword: String, file_path: String, ignore_case: u8) -> Result<Config, &'static str> {
        Ok(Config {
            query: keyword,
            file_path,
            ignore_case: ignore_case == 1,
        })
    }
}

/*
   根据关键字 query 返回匹配行数据
 */
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // 查询 包含 query 的字符串的行数
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

/*
   根据关键字 query 返回匹配行数据
   忽略大小写
 */
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase()
            .contains(&query.to_lowercase())
        )
        .collect()
}

/*
   根据关键字 query 返回匹配行数和行
 */
pub fn search_case_insensitive_w_l_n(query: &str, contents: &str) -> Vec<String> {
    // contents 根据每行的行号和数据转成一个hashmap
    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains(query))
        .map(|(line_number, line)| format!("{}# {}", line_number + 1, line))
        .collect()
}

/*
   根据关键字 query 返回匹配行数和行
   忽略大小写
 */
pub fn search_w_l_n(query: &str, contents: &str) -> Vec<String> {
    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| line.to_lowercase().contains(&query.to_lowercase()))
        .map(|(line_number, line)| format!("{}# {}", line_number + 1, line))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
