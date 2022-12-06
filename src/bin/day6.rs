use std::fs;

fn main() {
   let input = fs::read_to_string("day6.txt").unwrap();
   let mut start = 0;
   loop {
     if different(&input[start..]) {
       println!("Part 1: {}", start + 4);
       break;
     }
     start += 1;
   }
}

fn different(str: &str) -> bool {
  let s = str.as_bytes();
  s[0] != s[1] && s[0] != s[2] && s[0] != s[3] && s[1] != s[2] && s[1] != s[3] && s[2] != s[3]
}
