use std::fs::File;
use std::io::Read;

fn main() {
    let filename = "inputday3.txt";
    let file_data = lines_from_file(filename);
    //let file_data = ["vJrwpWtwJgWrhcsFMMfFFhFp","jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL","PmmdzqPrVvPwwTWBwg","wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn","ttgJtRGJQctTZtZT","CrZsJsPPZsGzwwsLwLmpwMDw"];
    let charrecters = ["a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","v","w","x","y","z","A","B","C","D","E","F","G","H","I","J","K","L","M","N","O","P","Q","R","S","T","U","V","W","X","Y","Z"];
    let mut items: Vec<&str> = Vec::new(); 
    let mut y = 0;
    let mut compartment1 = "a";
    let mut compartment2 = "a";

    //Part 1
    for x in &file_data {
        compartment1 = &x[0..x.len()/2];
        compartment2 = &x[x.len()/2..x.len()];
        while y < compartment2.len() {
            if compartment1.contains(&compartment2[y..y+1]) == true {
                items.push(&compartment2[y..y+1]);
                y = compartment2.len() + 1;
            }else{
                y = y + 1;
            }
        }
        y = 0;       
    }

    let mut total = 0;

    for x in &items {
        let temp = charrecters.iter().position(|&s| &s == x).unwrap() + 1;
        total = total + temp;
    }

    println!("{}", total);


    //Part 2
    println!("{}", day2(file_data, charrecters));

}

fn day2(file_data: Vec<String>,charrecters: [&str; 52]) -> i32 {
    let mut z = 0;
    let mut a = 0;
    let mut b = 0;
    let pt2total = 0;
    let mut pt2items: Vec<&str> = Vec::new(); 
    while z < file_data.len()-1 {
        let mut temp: Vec<&str> = Vec::new();         
        let bag1 = &file_data[z];
        let bag2 = &file_data[z+1];
        let bag3 = &file_data[z+2];
        println!("Bag 1 {}", bag1);
        println!("Bag 2 {}", bag2);
        println!("Bag 3 {}", bag3);
        while a < bag1.len() {
            if bag2.contains(&bag1[a..a+1]) == true {
                if bag3.contains(&bag1[a..a+1]) == true {
                    temp.push(&bag1[a..a+1]);
                    println!("Printing temp {:?}", temp);
                }
            }
            a = a + 1
        }

        while b < temp.len() {
            if temp.len() == 1 {
                pt2items.push(&temp[b]);
                println!("Printing True Values {:?}", pt2items);
                b = temp.len() + 1;
            }
            else if temp[b] == temp[b+1] {
                pt2items.push(&temp[b]);
                println!("Printing True Values {:?}", pt2items);
                b = temp.len() + 1;
            }
        }
        b = 0;
        a = 0;
        z = z + 3;
    }

    println!("{:?}", pt2items);

    let mut pt2total = 0;

    for x in &pt2items {
        let temp = charrecters.iter().position(|&s| &s == x).unwrap() + 1;
        pt2total = pt2total + temp;
    }

    pt2total.try_into().unwrap()

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