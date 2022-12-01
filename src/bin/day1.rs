use std::fs::File;
use std::io::{BufRead,BufReader};

fn main() {
   let reader = BufReader::new(File::open("day1-1.txt").unwrap());
   let data:Vec<i32> = reader.lines().map(|l| l.unwrap().parse::<i32>().unwrap_or(-1)).collect();
   let calories = split_delimited(&data, &-1);
   let mut elfs:Vec<i32> = calories.iter().map(|i| i.iter().sum::<i32>()).collect();
   elfs.sort_by(|a,b| b.cmp(a));
   println!("Max calories is {}", elfs[0]);
   println!("Big elfs {}", elfs[..3].iter().sum::<i32>())
}
       
