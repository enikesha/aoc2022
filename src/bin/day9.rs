use std::iter;
use std::fs::File;
use std::io::{BufRead,BufReader};
use std::collections::{HashSet, HashMap};


const STATES:[[usize;9];9] = [[0,1,2,3,4,5,6,7,8],
      			      [1,1,5,0,8,1,2,4,1],
			      [2,5,2,6,0,2,2,3,1],
			      [3,0,6,3,7,2,3,3,4],
			      [4,8,0,7,4,1,3,4,4],
			      [5,1,2,2,1,5,2,0,1],
			      [6,2,2,3,3,2,6,3,0],
			      [7,4,3,3,4,0,3,7,4],
			      [8,1,1,4,4,1,0,4,8]];
const MOVES:[[usize;9];9] =  [[0,0,0,0,0,0,0,0,0],
      			      [0,1,0,0,0,5,0,0,8],
			      [0,0,2,0,0,5,6,0,0],
			      [0,0,0,3,0,0,6,7,0],
			      [0,0,0,0,4,0,0,7,8],
			      [0,5,5,0,0,5,2,0,1],
			      [0,0,6,6,0,2,6,3,0],
			      [0,0,0,7,7,0,3,7,4],
			      [0,8,0,0,8,1,0,4,8]];

const D:[(i32,i32);9] = [(0,0),(0,-1),(1,0),(0,1),(-1,0),(1,-1),(1,1),(-1,1),(-1,-1)];
const DIRS:&str = ".URDL";
const KNOTS:[char;9] = ['1','2','3','4','5','6','7','8','9'];

fn main() {
   let reader = BufReader::new(File::open("day9.txt").unwrap());
   let moves = reader.lines().flat_map(|l| {
       let l = l.unwrap();
       let dir = l.chars().next().unwrap();
       let count = l[2..].parse::<usize>().unwrap();
       iter::repeat(DIRS.find(dir).unwrap()).take(count)
   }).collect::<Vec<_>>();

   let mut head = (0,0);
   let mut init1 = [(0,0,0); 1];
   let mut init2 = [(0,0,0); 9];
   let mut steps1 = HashSet::new();
   let mut steps2 = HashSet::new();
   let mut tail:HashMap<(i32,i32), char> = HashMap::new();
   for (s, mov) in moves.into_iter().enumerate() {
     //println!("Move {s}: {mov}");
     let mut field:HashMap<(i32,i32), char> = HashMap::new();
     field.insert((0,0), 's');
     let mut cmov = mov;
     for (i, knot) in init1.iter_mut().enumerate() {
     	 let (state, x, y) = *knot;
	 knot.0 = STATES[state][cmov];
	 cmov = MOVES[state][cmov];
 	 let (dx,dy) = D[cmov];
	 knot.1 = x+dx;
	 knot.2 = y+dy;
     }
     cmov = mov;
     for (i, knot) in init2.iter_mut().enumerate() {
     	 let (state, x, y) = *knot;
	 knot.0 = STATES[state][cmov];
	 cmov = MOVES[state][cmov];
 	 let (dx,dy) = D[cmov];
	 knot.1 = x+dx;
	 knot.2 = y+dy;
	 field.insert((knot.1,knot.2), KNOTS[i]);
     }
     tail.insert((init2[8].1,init2[8].2), '#');
     head.0 = head.0 + D[mov].0;
     head.1 = head.1 + D[mov].1;     
     field.insert(head, 'H');
     //print_field(&field);
     steps1.insert((init1[0].1, init1[0].2));
     steps2.insert((init2[8].1, init2[8].2));
   }
   tail.insert((0,0), 's');
   //println!("Tail");
   //print_field(&tail);
   println!("Part 1: {:?}, Part 2: {}", steps1.len(), steps2.len()); 
}

fn print_field(field : &HashMap<(i32,i32), char>) {
  let minx = field.keys().map(|p| p.0).min().unwrap();
  let miny = field.keys().map(|p| p.1).min().unwrap();
  let maxx = field.keys().map(|p| p.0).max().unwrap();
  let maxy = field.keys().map(|p| p.1).max().unwrap();

  for y in miny..maxy+1 {
    let mut row = iter::repeat('.').take((maxx-minx+1).try_into().unwrap()).collect::<Vec<_>>();
    for x in minx..maxx+1 {
      let idx:usize = (x-minx).try_into().unwrap();
      row[idx] = *field.get(&(x,y)).unwrap_or(&'.');
    }
    println!("{}", row.iter().cloned().collect::<String>());
  }
}

