use std::{
    collections::HashMap,
    fmt::{write, Display},
};

#[derive(Debug)]
struct WordCounter(HashMap<String, i64>);

impl WordCounter {
    fn new() -> WordCounter {
        WordCounter(HashMap::new())
    }
    fn count() {}
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
    let w = WordCounter::new();
    println!("{}", w)
}
