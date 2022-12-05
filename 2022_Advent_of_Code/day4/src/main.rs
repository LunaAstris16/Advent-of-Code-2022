use std::fs::File;
use std::io::Read;

fn main() {
    let filename = "inputday4.txt";
    let file_data = lines_from_file(filename);
    let mut total = 0;
  
    for x in file_data {
      let temp: Vec<&str> = x.split(",").collect();
      let mut nums: Vec<i32> = Vec::new();
      let num1: Vec<&str> = temp[0].split("-").collect();
      let num2: Vec<&str> = temp[1].split("-").collect();
      nums.push(num1[0].parse::<i32>().unwrap());
      nums.push(num1[1].parse::<i32>().unwrap());
      nums.push(num2[0].parse::<i32>().unwrap());
      nums.push(num2[1].parse::<i32>().unwrap());
      let number1: Vec<i32> = (nums[0]..nums[1]+1).collect();
      let number2: Vec<i32> = (nums[2]..nums[3]+1).collect();
      if number1.len() <= number2.len() {
        if number1.iter().all(|e| number2.contains(e)) == true{
          total = total + 1;
        }
      } else {
        if number2.iter().all(|e| number1.contains(e)) == true{
          total = total + 1;
        }
      }
    }
  
  println!("{}", total);

  //part 2
    total = 0;
    let filename = "inputday4.txt";
    let file_data = lines_from_file(filename);
  
    for x in file_data {
      let temp: Vec<&str> = x.split(",").collect();
      let mut nums: Vec<i32> = Vec::new();
      let num1: Vec<&str> = temp[0].split("-").collect();
      let num2: Vec<&str> = temp[1].split("-").collect();
      nums.push(num1[0].parse::<i32>().unwrap());
      nums.push(num1[1].parse::<i32>().unwrap());
      nums.push(num2[0].parse::<i32>().unwrap());
      nums.push(num2[1].parse::<i32>().unwrap());
      let number1: Vec<i32> = (nums[0]..nums[1]+1).collect();
      let number2: Vec<i32> = (nums[2]..nums[3]+1).collect();
      if number1.len() <= number2.len() {
        let mut i = 0;
        while i < number1.len() {
          if number2.contains(&number1[i]) == true{
            total = total + 1;
            i = number1.len()+1;
          }
          else {
            i = i + 1;
          }
        }
      } else {
        let mut i = 0;
        while i < number2.len() {
          if number1.contains(&number2[i]) == true{
            total = total + 1;
            i = number2.len()+1;
          }
          else {
            i = i + 1;
          }
        }
      }
    }
  
    println!("{}", total);
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
