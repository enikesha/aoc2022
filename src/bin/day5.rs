use std::fs::File;
use std::io::{BufRead,BufReader};

fn main() {
   let reader = BufReader::new(File::open("test.txt").unwrap());
   let mut lines = reader.lines().map(|l| l.unwrap());
   let mut stacks:Vec<Vec<char>> = Vec::new();
   loop {
     let line = lines.next().unwrap();
     if line.starts_with(" 1") {
       lines.next();
       break;
     }
     for (i, c) in line.as_bytes().chunks(2).enumerate() {
       if i % 2 == 1 { continue; }
       println!("{:?}", c);
     }
   }

}
