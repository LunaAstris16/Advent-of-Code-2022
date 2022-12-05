use std::fs::File;
use std::io::Read;

fn main() {
    let filename = "testingday4.txt";
    let file_data = lines_from_file(filename);
      
    for x in file_data {
        let temp: Vec<&str> = x.split(",").collect();
        let number1: Vec<i32> = (temp[0][0..1]..temp[0][2..3]).collect();
        println!("{:?}", number1);
    }
}

fn lines_from_file(filename: &str) -> Vec<String> {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read!");
    let lines: Vec<String> = file_contents.split("\n")
        .map(|s: &str| s.to_string())
        .collect();
    lines
}
