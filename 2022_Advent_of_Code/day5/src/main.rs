use std::fs::File;
use std::io::Read;

fn main() {
  let filename = "inputday5.txt";
  let file_data = lines_from_file(filename);

  let mut cargocrates: Vec<Vec<&str>> = Vec::new();
  let mut holdingcrate1: Vec<&str> = Vec::new();
  let mut holdingcrate2: Vec<&str> = Vec::new();
  let mut holdingcrate3: Vec<&str> = Vec::new();
  let mut holdingcrate4: Vec<&str> = Vec::new();
  let mut holdingcrate5: Vec<&str> = Vec::new();
  let mut holdingcrate6: Vec<&str> = Vec::new();
  let mut holdingcrate7: Vec<&str> = Vec::new();
  let mut holdingcrate8: Vec<&str> = Vec::new();
  let mut holdingcrate9: Vec<&str> = Vec::new();
  let mut tempholder = 0;
  
  let mut x = 0;
  
  while x < file_data.len()-1 {
    let temp: Vec<&str> = file_data[x].split(" ").collect();
    if temp.len() < 2 {
      tempholder = x - 1; 
      x = file_data.len() + 1;
    }
    x = x + 1;
  }

  let mut y = 0;
  while y < tempholder {
    let temp: Vec<&str> = file_data[y].split("    ").collect();
    let mut cargolist: Vec<&str> = Vec::new();
    //println!("{:?}", temp);
    for i in temp {
      if i.len() > 3 {
        let internalsplit: Vec<&str> = i.split(" ").collect();
        let mut z = 0;
        while z < internalsplit.len(){
          cargolist.push(internalsplit[z]);
          z = z + 1;
        }
      } else {
        cargolist.push(i);
      }
    }
    if cargolist[0].is_empty() == false {
      holdingcrate1.push(cargolist[0]);
    }
    if cargolist[1].is_empty() == false {
      holdingcrate2.push(cargolist[1]);
    }
    if cargolist[2].is_empty() == false {
      holdingcrate3.push(cargolist[2]);
    }
    if cargolist[3].is_empty() == false {
      holdingcrate4.push(cargolist[3]);
    }
    if cargolist[4].is_empty() == false {
      holdingcrate5.push(cargolist[4]);
    }
    if cargolist[5].is_empty() == false {
      holdingcrate6.push(cargolist[5]);
    }
    if cargolist[6].is_empty() == false {
      holdingcrate7.push(cargolist[6]);
    }
    if cargolist[7].is_empty() == false {
      holdingcrate8.push(cargolist[7]);
    }
    if cargolist[8].is_empty() == false {
      holdingcrate9.push(cargolist[8]);
    }
    y = y + 1;
  }

  holdingcrate1.reverse();
  holdingcrate2.reverse();
  holdingcrate3.reverse();
  holdingcrate4.reverse();
  holdingcrate5.reverse();
  holdingcrate6.reverse();
  holdingcrate7.reverse();
  holdingcrate8.reverse();
  holdingcrate9.reverse();

  cargocrates.push(holdingcrate1);
  cargocrates.push(holdingcrate2);
  cargocrates.push(holdingcrate3);
  cargocrates.push(holdingcrate4);
  cargocrates.push(holdingcrate5);
  cargocrates.push(holdingcrate6);
  cargocrates.push(holdingcrate7);
  cargocrates.push(holdingcrate8);
  cargocrates.push(holdingcrate9);

  let mut z = tempholder + 2;
  while z < file_data.len() {
    let instruction: Vec<&str> = file_data[z].split(" ").collect();
    let pushamount: i32 = instruction[1].parse().unwrap_or(0);
    let pushtoint = instruction[5].parse().unwrap_or(0);
    let pushfromint = instruction[3].parse().unwrap_or(0);
    let temp = &cargocrates[pushtoint - 1];
    let mut pushto: Vec<&str> = temp.to_vec();
    let ttemp = &cargocrates[pushfromint - 1];
    let mut pushfrom: Vec<&str> = ttemp.to_vec();
    let mut temp = 0; 
    while temp < pushamount {
      pushto.push(pushfrom[pushfrom.len() -1]);
      pushfrom.remove(pushfrom.len()-1);
      temp = temp + 1;
    }
    cargocrates[pushtoint - 1] = pushto.to_vec();
    cargocrates[pushfromint -1] = pushfrom.to_vec();
    
    z = z + 1;
  }

  println!("First Part Answer");
  println!("{:?}", cargocrates[0][cargocrates[0].len() - 1]);
  println!("{:?}", cargocrates[1][cargocrates[1].len() - 1]);
  println!("{:?}", cargocrates[2][cargocrates[2].len() - 1]);
  println!("{:?}", cargocrates[3][cargocrates[3].len() - 1]);
  println!("{:?}", cargocrates[4][cargocrates[4].len() - 1]);
  println!("{:?}", cargocrates[5][cargocrates[5].len() - 1]);
  println!("{:?}", cargocrates[6][cargocrates[6].len() - 1]);
  println!("{:?}", cargocrates[7][cargocrates[7].len() - 1]);
  println!("{:?}", cargocrates[8][cargocrates[8].len() - 1]);
  println!("Secound Part Answer");

  let filename = "inputday5.txt";
  let file_data = lines_from_file(filename);

  let mut cargocrates: Vec<Vec<&str>> = Vec::new();
  let mut holdingcrate1: Vec<&str> = Vec::new();
  let mut holdingcrate2: Vec<&str> = Vec::new();
  let mut holdingcrate3: Vec<&str> = Vec::new();
  let mut holdingcrate4: Vec<&str> = Vec::new();
  let mut holdingcrate5: Vec<&str> = Vec::new();
  let mut holdingcrate6: Vec<&str> = Vec::new();
  let mut holdingcrate7: Vec<&str> = Vec::new();
  let mut holdingcrate8: Vec<&str> = Vec::new();
  let mut holdingcrate9: Vec<&str> = Vec::new();
  let mut tempholder = 0;
  
  let mut x = 0;
  
  while x < file_data.len()-1 {
    let temp: Vec<&str> = file_data[x].split(" ").collect();
    if temp.len() < 2 {
      tempholder = x - 1; 
      x = file_data.len() + 1;
    }
    x = x + 1;
  }

  let mut y = 0;
  while y < tempholder {
    let temp: Vec<&str> = file_data[y].split("    ").collect();
    let mut cargolist: Vec<&str> = Vec::new();
    println!("{:?}", temp);
    for i in temp {
      if i.len() > 3 {
        let internalsplit: Vec<&str> = i.split(" ").collect();
        let mut z = 0;
        while z < internalsplit.len(){
          cargolist.push(internalsplit[z]);
          z = z + 1;
        }
      } else {
        cargolist.push(i);
      }
    }
    if cargolist[0].is_empty() == false {
      holdingcrate1.push(cargolist[0]);
    }
    if cargolist[1].is_empty() == false {
      holdingcrate2.push(cargolist[1]);
    }
    if cargolist[2].is_empty() == false {
      holdingcrate3.push(cargolist[2]);
    }
    if cargolist[3].is_empty() == false {
      holdingcrate4.push(cargolist[3]);
    }
    if cargolist[4].is_empty() == false {
      holdingcrate5.push(cargolist[4]);
    }
    if cargolist[5].is_empty() == false {
      holdingcrate6.push(cargolist[5]);
    }
    if cargolist[6].is_empty() == false {
      holdingcrate7.push(cargolist[6]);
    }
    if cargolist[7].is_empty() == false {
      holdingcrate8.push(cargolist[7]);
    }
    if cargolist[8].is_empty() == false {
      holdingcrate9.push(cargolist[8]);
    }
    y = y + 1;
  }

  holdingcrate1.reverse();
  holdingcrate2.reverse();
  holdingcrate3.reverse();
  holdingcrate4.reverse();
  holdingcrate5.reverse();
  holdingcrate6.reverse();
  holdingcrate7.reverse();
  holdingcrate8.reverse();
  holdingcrate9.reverse();

  cargocrates.push(holdingcrate1);
  cargocrates.push(holdingcrate2);
  cargocrates.push(holdingcrate3);
  cargocrates.push(holdingcrate4);
  cargocrates.push(holdingcrate5);
  cargocrates.push(holdingcrate6);
  cargocrates.push(holdingcrate7);
  cargocrates.push(holdingcrate8);
  cargocrates.push(holdingcrate9);

  let mut z = tempholder + 2;
  while z < file_data.len() {
    let instruction: Vec<&str> = file_data[z].split(" ").collect();
    let pushamount: i32 = instruction[1].parse().unwrap_or(0);
    let pushtoint = instruction[5].parse().unwrap_or(0);
    let pushfromint = instruction[3].parse().unwrap_or(0);
    let temp = &cargocrates[pushtoint - 1];
    let mut pushto: Vec<&str> = temp.to_vec();
    let ttemp = &cargocrates[pushfromint - 1];
    let mut pushfrom: Vec<&str> = ttemp.to_vec();
    let mut temp = 0; 
    let mut templist: Vec<&str> = Vec::new();
    while temp < pushamount {
      templist.push(pushfrom[pushfrom.len() - 1]);
      pushfrom.remove(pushfrom.len()-1);
      temp = temp + 1;
    }
    templist.reverse();
    let mut temp = 0;
    while temp < templist.len().try_into().unwrap() {
      pushto.push(templist[temp]);
      temp = temp + 1;
    }
    cargocrates[pushtoint - 1] = pushto.to_vec();
    cargocrates[pushfromint -1] = pushfrom.to_vec();
    
    z = z + 1;
  }
  
  println!("{:?}", cargocrates[0][cargocrates[0].len() - 1]);
  println!("{:?}", cargocrates[1][cargocrates[1].len() - 1]);
  println!("{:?}", cargocrates[2][cargocrates[2].len() - 1]);
  println!("{:?}", cargocrates[3][cargocrates[3].len() - 1]);
  println!("{:?}", cargocrates[4][cargocrates[4].len() - 1]);
  println!("{:?}", cargocrates[5][cargocrates[5].len() - 1]);
  println!("{:?}", cargocrates[6][cargocrates[6].len() - 1]);
  println!("{:?}", cargocrates[7][cargocrates[7].len() - 1]);
  println!("{:?}", cargocrates[8][cargocrates[8].len() - 1]);
  
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
