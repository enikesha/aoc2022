use std::fs::File;
use std::io::{BufRead,BufReader};
use std::collections::HashMap;

fn main() {
   let scores = HashMap::from([
     ("A X", 4),
     ("A Y", 8),
     ("A Z", 3),
     ("B X", 1),
     ("B Y", 5),
     ("B Z", 9),
     ("C X", 7),
     ("C Y", 2),
     ("C Z", 6),
   ]);
   let moves = HashMap::from([
     ("A X", "A Z"),
     ("A Y", "A X"),
     ("A Z", "A Y"),
     ("B X", "B X"),
     ("B Y", "B Y"),
     ("B Z", "B Z"),
     ("C X", "C Y"),
     ("C Y", "C Z"),
     ("C Z", "C X"),
   ]);
   let reader = BufReader::new(File::open("day2.txt").unwrap());
   let data:Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
   //println!("input: {:?}", data);
   let rounds = data.iter().map(|m| scores.get(&m.as_str()).unwrap());
   println!("Part 1: {}", rounds.sum::<i32>());
   let rounds = data.iter().map(|m| *scores.get(moves.get(&m.as_str()).unwrap()).unwrap());
   //let rounds = data.iter().map(|m| moves.get(m.as_str()).unwrap());
   //println!("{:?}", rounds.collect::<Vec<i32>>());
   println!("Part 2: {}", rounds.sum::<i32>());
}