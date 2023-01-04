use std::fs::File;
use std::io::Read;

fn main() {
  let filename = "inputday6.txt";
  let file_data = lines_from_file(filename);
  let longstring = &file_data[0];
  let mut index = 4;
  while index < longstring.len() {
    let next_letter = &longstring[index..index+1];
    if longstring[index-4..index-1].contains(next_letter) == false{
      let smallstring = &longstring[index-4..index];
      let mut holding = 0;
      let mut contains = false;
      while holding < smallstring.len() {
        let letter = &smallstring[holding..holding+1];
        let otherpart = format!("{}{}", &smallstring[0..holding], &smallstring[holding+1..smallstring.len()]);
        if otherpart.contains(letter) == true{
          contains = true;
        }
        holding = holding + 1;
      }
      if contains == false {
        println!("Part 1");
        println!("{}", index);
        index = longstring.len()+1;
      }
    }
    index = index + 1;
  }

  println!("Part 2");
  
  let filename = "inputday6.txt";
  let file_data = lines_from_file(filename);
  let longstring = &file_data[0];
  let mut index = 14;
  while index < longstring.len() {
    let next_letter = &longstring[index..index+1];
    if longstring[index-13..index].contains(next_letter) == false{
      let smallstring = &longstring[index-13..index];
      let mut holding = 0;
      let mut contains = false;
      while holding < smallstring.len() {
        let letter = &smallstring[holding..holding+1];
        let otherpart = format!("{}{}", &smallstring[0..holding], &smallstring[holding+1..smallstring.len()]);
        if otherpart.contains(letter) == true{
          contains = true;
          holding = smallstring.len() + 1;
        } else {
          contains = false;
        }
        holding = holding + 1;
      }
      if contains == false {
        index = index + 1;
        println!("{}", index);
        index = longstring.len()+1;
      }
      index = index + 1;
    }
    else {
      index = index + 1;
     
    }
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
