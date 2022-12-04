use std::fs::File;
use std::io::{BufRead,BufReader};

fn main() {
   let reader = BufReader::new(File::open("day4.txt").unwrap());
   let lines:Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
   //println!("{:?}", lines);

   let data = lines.iter().map(|l| {
     let pair = l.split_once(',').unwrap();
     let elf1 = pair.0.split_once('-').unwrap();
     let elf2 = pair.1.split_once('-').unwrap();
     (elf1.0.parse::<u32>().unwrap(),
      elf1.1.parse::<u32>().unwrap(),
      elf2.0.parse::<u32>().unwrap(),
      elf2.1.parse::<u32>().unwrap())
   }).collect::<Vec<_>>();

   //println!("{:?}", data);
   println!("Part 1: {}", data.iter().filter(contains).count());
   println!("Part 2: {:?}", data.iter().filter(overlap).count());//.map(|p| *p).collect::<Vec<_>>());
}

fn contains(p : &&(u32, u32, u32, u32)) -> bool {
  (p.0 >= p.2 && p.1 <= p.3) || (p.2 >= p.0 && p.3 <= p.1)
}

fn overlap(p : &&(u32, u32, u32, u32)) -> bool {
  !(p.1 < p.2 || p.0 > p.3)
}
