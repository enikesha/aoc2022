use std::fs::File;
use std::io::{BufRead,BufReader};
use std::collections::HashSet;

fn prio(c:u8) -> u32 { (if c > 96 { c - 96 } else { c - 38 }) as u32 }

fn main() {
   let reader = BufReader::new(File::open("day3.txt").unwrap());
   let data = reader.lines().map(|l| l.unwrap()).collect::<Vec<_>>();
   let rucsacs = data.iter().map(|l| {
     let (p1, p2) = l.split_at(l.len()/2);
     let p1 = p1.as_bytes().iter().collect::<HashSet<_>>();
     let p2 = p2.as_bytes().iter().collect::<HashSet<_>>();
     prio(**p1.intersection(&p2).next().unwrap())
   });
   println!("Part 1 {:?}", rucsacs.sum::<u32>());

   let groups = data.chunks_exact(3).map(|c| {
     let p1 = c[0].as_bytes().iter().map(|&i| i).collect::<HashSet<_>>();
     let p2 = c[1].as_bytes().iter().map(|&i| i).collect::<HashSet<_>>();
     let p3 = c[2].as_bytes().iter().collect::<HashSet<_>>();
     prio(**p1.intersection(&p2).collect::<HashSet<_>>().intersection(&p3).next().unwrap())
   });  
   println!("Part 2 {:?}", groups.sum::<u32>());
}