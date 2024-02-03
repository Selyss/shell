use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn get_size(file_path: &str) -> usize {
    let path = Path::new(file_path);
    let file = File::open(&path).expect("Unable to read file");
    let reader = BufReader::new(file);
    let mut total_size = 0;
    for line in reader.lines() {
        total_size += line.unwrap().as_bytes().len();
    }
    return total_size;
}
pub fn filesize(file: &str) -> &str {
    let total_size = get_size(file);
    let size_in_mb = (total_size as f64) / (1024.0 * 1024.0);
    let size_string = format!("{:.2} MB", size_in_mb);
    // FIXME: pick either kb, mb, or gb
    println!("{}", total_size);
    println!("{}", size_in_mb);
    println!("{}", size_string);
    return "";
}
