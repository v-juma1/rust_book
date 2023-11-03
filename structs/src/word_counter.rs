use std::{
    collections::HashMap,
    env,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Debug)]
struct WordCounter(HashMap<String, i64>);

impl WordCounter {
    fn new() -> WordCounter {
        WordCounter(HashMap::new())
    }
    fn count(&mut self, line: &str) {
        for word in line.split(" ").collect::<Vec<&str>>() {
            self.0
                .entry(word.to_string())
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
    }
}

impl Display for WordCounter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{{")?;

        for (k, v) in &self.0 {
            writeln!(f, "  {}: {}", k, v)?;
        }

        write!(f, "}}")
    }
}

pub fn run() {
    let mut w = WordCounter::new();
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => (),
        _ => panic!("参数错误！"),
    }
    let path = Path::new(&args[1]);
    let f = File::open(path).expect("file does not exist!");
    let reader: BufReader<File> = BufReader::new(f);
    for line in reader.lines() {
        w.count(&line.expect("can not read the file:{}"))
    }

    println!("{}", w)
}
