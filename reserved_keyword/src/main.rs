use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::collections::HashMap;

fn main() {
    
    let belge = File::open("keyword.txt").expect("Belge açılmıyor!");
  
    let buf_reader = BufReader::new(belge);
    
    let mut tablo = HashMap::new();

    for line in buf_reader.lines() {
        let parca: Vec<String> = line.unwrap().split("-").map(|s: &str| s.to_string()).collect(); 
        let ilk = String::from(&parca[0]);
        let son = String::from(&parca[1]);
        let i = ilk.trim();
        let s = son.trim();
        tablo.insert(String::from(i), String::from(s));
    }

    //println!("{:?}", tablo);

    // Bu noktadan sonra hash sorgusu yapılacak

    let mut x = String::new();
    
    println!("Öğrenmek istediğiniz kelimeyi giriniz:");

    io::stdin().read_line(&mut x).expect("Error reading input");
    
    let input: String = x.trim().parse().expect("Error parsing number");

    //println!("{:?}", input);
    
    match tablo.get(&input) {
                Some(kelime) => println!("{}: {}", input, kelime),
                None => println!("Tabloda böyle bir anahtar kelime yok"),
     };
     
}