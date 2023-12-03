use std::{fs, collections::{HashMap, HashSet}};
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref RE: Regex = Regex::new(r"\d+").unwrap();
}
fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let mut nums = HashMap::new();
    let board = contents.lines().map(|x| x.chars().collect()).collect::<Vec<Vec<char>>>();
    let mut part1 = 0;
    for (y,line) in contents.lines().enumerate() {
        let y = y as i32;
        for m in RE.captures_iter(line) {
            let n = m[0].parse::<usize>().unwrap();
            let start: i32 = m.get(0).unwrap().start() as i32;
            let end: i32 = m.get(0).unwrap().end() as i32;

            for x in start..end {
                nums.insert((y,x), (n, start, end));
            }

            if !is_clear(&board, y, start-1) 
            || !is_clear(&board, y, end) 
            || (start-1..=end).any(|x: i32| !is_clear(&board, y+1, x) || !is_clear(&board, y-1, x)) {
                part1 += n;
                continue;
            }
        }
    }

    // println!("{:?}", nums);
    let mut part2 = 0;
    for (y,line) in board.iter().enumerate() {
        let y = y as i32;
        for (x,c) in line.iter().enumerate() {
            let x = x as i32;
            if *c != '*' {
                continue;
            }
            let mut adj = 0;
            let mut adj_tot = 1;
            let mut seen = HashSet::new();
            for dx in -1..2 {
                for dy in -1..2 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    if nums.contains_key(&(y+dy,x+dx)) && !seen.contains(&(y+dy,x+dx)) {
                        let (n,start,end) = nums.get(&(y+dy,x+dx)).unwrap();
                        for i in *start..*end {
                            seen.insert((y+dy,i));
                        }
                        adj += 1;
                        adj_tot *= n;
                    }
                }
            }
            if adj == 2 {
                part2 += adj_tot;
            }
        }
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn is_clear(board: &Vec<Vec<char>>, y:i32, x:i32) -> bool {
    y<0 || y>=board.len() as i32 || x<0 || x>=board[0].len() as i32 || board[y as usize][x as usize] == '.'
}