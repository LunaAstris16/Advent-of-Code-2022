use std::fs::File;
use std::io::Read;

fn main() {
  let filename = "inputday2.txt";
  let file_data = lines_from_file(filename);
  let mut score = 0;
  let constants = [4,8,3,1,5,9,7,2,6] //use for part 1
  //let constants = [3,4,8,1,5,9,2,6,7] use for part 2
  

  let mut x = 0;

  while x < file_data.len()-1 {
    println!("{}", x);
    let newtemp: Vec<&str> = file_data[x].split(" ").collect();
    let oppenent = newtemp[0];
    let player = newtemp[1];
    println!("{}", newtemp[0]);
    println!("{}", newtemp[1]);
    if oppenent == "A"{
      if player == "X"{
        score = score + constants[0];
      }else if player == "Y"  {
        score = score + constants[1];        
      }else if player == "Z" {
        score = score + constants[2];    
      }
    }else if oppenent == "B" {
      if player == "X"{
        score = score + constants[3];        
      }else if player == "Y"  {
        score = score + constants[4];        
      }else if player == "Z" {
        score = score + constants[5];    
      }
    }else if oppenent == "C" {
      if player == "X"{
        score = score + constants[6];        
      }else if player == "Y"  {
        score = score + constants[7];         
      }else if player == "Z" {
        score = score + constants[8];     
      }
    }
    
    x = x + 1;
  }

  println!("{}", score);

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