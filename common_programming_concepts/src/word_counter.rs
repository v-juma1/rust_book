// use std::collections::HashMap;

// #[derive(Debug)]
// struct WordCounter(HashMap<String, u64>);

// impl WordCounter {
//     fn new() -> WordCounter {
//         WordCounter(HashMap::new())
//     }
//     fn display(&self) {
//         for (k, v) in self.0.iter() {
//             println!("{}:{}", k, v)
//         }
//     }

//     fn caculate(&self, word: &str) {
//         println!("{word}");
//         self.display();
//     }
// }

// pub fn run() {
//     let w = WordCounter::new();

//     let r = w.caculate("adcd");
// }
