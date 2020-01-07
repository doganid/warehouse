use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::collections::HashMap;

fn main() {
    
    let file = File::open("keyword.txt").expect("Unable to access file!");
  
    let buf_reader = BufReader::new(file);
    
    let mut mapp = HashMap::new();

    for line in buf_reader.lines() {
        let parca: Vec<String> = line.unwrap().split("-").map(|s: &str| s.to_string()).collect(); 
        let ilk = String::from(&parca[0]);
        let son = String::from(&parca[1]);
        let i = ilk.trim();
        let s = son.trim();
        mapp.insert(String::from(i), String::from(s));
    }

    let mut x = String::new();
    
    println!("Enter the keyword you'd like to know:");

    io::stdin().read_line(&mut x).expect("Error reading input word!");
    
    let input: String = x.trim().parse().expect("Error parsing input word!");
 
    match mapp.get(&input) {
                Some(kelime) => println!("{}: {}", input, kelime),
                None => println!("Tabloda böyle bir anahtar kelime yok"),
     };  
}
