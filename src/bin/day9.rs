use std::fs::File;
use std::io::{BufRead,BufReader};

const states:[[i32;4];9]= [[3,4,1,2], [0,8,3,5], [6,0,5,2], [3,7,0,6], [7,4,8,0], [2,1,1,2],  [3,3,2,2],  [3,4,4,3],  [4,4,1,1]];
const dx:[[i32;4];9]   = [[0,0,0,0], [0,0,0,0], [0,0,0,-1],[0,0,0,0], [0,1,0,0], [0,0,-1,-1],[-1,0,0,-1],[1,1,0,0],  [0,1,1,0]];
const dy:[[i32;4];9]    = [[0,0,0,0], [0,0,1,0], [0,0,0,0], [-1,0,0,0],[0,0,0,0], [0,0,1,1],  [-1,0,0,-1],[-1,-1,0,0],[0,1,1,0]];

fn main() {
   let reader = BufReader::new(File::open("test.txt").unwrap());
   let mut rope = (0,vec![(0,0)]);

}

fn step(rope:&(i8,Vec::<(i32,i32)>), mve:i8) -> &(i8, Vec::<(i32,i32)>){
  let (state, mut positions) = rope;
  let tail = positions.last().unwrap();
  positions.push((tail.0 + dx[state][mve], tail.1 + dy[state][mve]));
  (states[state][mve], positions)
}