use std::fs::File;
use std::io::Read;

fn main() {
  let filename = "inputday1.txt";
  let file_data = vec![lines_from_file(filename)];
  let mut high_values: Vec<i32> = Vec::new();
  let mut holding_number = 0;
  
  for i in &file_data {
    if i.is_empty() == true {
      high_values.push(holding_number);
    } else {
      
    }
    
    println!("{:?}", i);
  };
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
