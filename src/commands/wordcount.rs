use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn count_words(file_path: &str) -> usize {
    let path = Path::new(file_path);
    let file = File::open(&path).expect("Unable to read file");
    let reader = BufReader::new(file);
    let mut word_count = 0;
    for line in reader.lines() {
        word_count += line.unwrap().split_whitespace().count(); // this may explode
    }
    return word_count;
}
pub fn wordcount(file: &str) -> &str {
    let words = count_words(file);
    println!("{}", words);
    return "";
}
