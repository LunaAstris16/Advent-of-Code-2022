use std::fs::File;
use std::io::Read;

fn main() {
  let filename = "inputday1.txt";
  let file_data = lines_from_file(filename);
  let mut high_values: Vec<i32> = Vec::new();
  let mut holding_number = 0;
  
  for i in &file_data {
    if i.is_empty() == true {
      high_values.push(holding_number);
      holding_number = 0;
    } else {
      let temp: i32 = i.parse().expect("Not a number!");
      holding_number = holding_number + temp;
    }
  };

  high_values.sort();
  high_values.reverse();
  
  let high: &i32 = &high_values[0];
  let secound: &i32 = &high_values[1];
  let third: &i32 = &high_values[2];
  
  println!("{}", high+third+secound); //Answer to secound problem
  println!("{}", high); // Answer to first problem
  println!("{}", secound);
  println!("{}", third);
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
