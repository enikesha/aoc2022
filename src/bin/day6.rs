use std::fs;
use std::collections::HashSet;

fn main() {
   let input = fs::read_to_string("day6.txt").unwrap();
   let mut start = 0;
   let mut found = 0;
   loop {
     if different(&input[start..start+4]) && (found & 1 == 0){
       println!("Part 1: {}", start + 4);
       found |= 1
     }
     if different(&input[start..start+14]) && (found & 2 == 0) {
       println!("Part 2: {}", start + 14);
       found |= 2;
     }
     if found == 3 { break; }
     start += 1;
   }
}

fn different(str: &str) -> bool {
  let bytes = str.as_bytes().iter().collect::<HashSet<_>>();
  bytes.len() == str.len()
}
