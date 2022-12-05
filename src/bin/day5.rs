use std::fs::File;
use std::io::{BufRead,BufReader};

fn main() {
   let reader = BufReader::new(File::open("day5.txt").unwrap());
   let mut lines = reader.lines().map(|l| l.unwrap());
   let mut stacks:Vec<Vec<char>> = Vec::new();
   loop {
     let line = lines.next().unwrap();
     if line.starts_with(" 1") {
       lines.next();
       break;
     }
     for (i, c) in line.chars().collect::<Vec<char>>().chunks(2).enumerate() {
       if i % 2 == 1 { continue; }
       let i = i / 2;
       if stacks.get(i).is_none() { stacks.push(Vec::new()); }
       let c = c[1];
       let stack = stacks.get_mut(i).unwrap();
       if c != ' ' { stack.insert(0, c); }
     }
   }
   for line in lines {
     let count = parse(&line, &"move ");
     let from = parse(&line, &" from ") - 1;
     let to = parse(&line, &" to ") - 1;
     //println!("{:?}", stacks);
     //println!("Move {count} from {from} to {to}");
     let from_stack = stacks.get(from).unwrap();
     let (left, moved) = from_stack.split_at(from_stack.len()-count);
     let moved = moved.to_vec();
     //let moved = moved.iter().rev().collect::<Vec<_>>();
     stacks[from] = left.to_vec();
     let to_stack = stacks.get_mut(to).unwrap();
     to_stack.extend(moved);
   }
   println!("{}", stacks.iter().map(|s| s.last().unwrap()).collect::<String>());
}

fn parse(line: &str, from: &str) -> usize {
  let from = line.find(from).unwrap() + from.len();
  let from_end = match line[from..].find(' ') {
    None => line.len(),
    Some(i) => i + from
  };

  line[from..from_end].parse::<usize>().unwrap()
}
