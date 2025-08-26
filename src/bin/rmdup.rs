use std::collections::HashMap;
use std::fs;
use std::io::stdin;

fn main() {
    let inp = stdin();
    let mut line = String::new();
    let mut ht = HashMap::new();
    let mut rmlist = Vec::new();
    loop {
        if inp.read_line(&mut line).unwrap() == 0 {
            break;
        }
        line = line.strip_suffix("\n").unwrap().to_string();
        let mut spl = line.split(" ");
        let hash = spl.nth(0).unwrap();
        let fname = spl.nth(1).unwrap();
        if ht.contains_key(&hash.to_string()) {
            rmlist.push(fname.to_string());
        } else {
            ht.insert(hash.to_string(), fname.to_string());
        }
        line.clear();
    }
    println!("Removing {} files", rmlist.len());
    for file in rmlist {
        let ret = fs::remove_file(file.clone());
        if ret.is_err() {
            println!("Failed on file {} with error {}", file, ret.unwrap_err());
        }
    }
    println!("Done");
}
