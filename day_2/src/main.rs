use std::fs;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\d+) ([rgb])").unwrap();
}
fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let mut part1 = 0;
    let mut part2 = 0;
    for (i,line) in contents.lines().enumerate() {
        let mut v = [0; 3];
        for cap in RE.captures_iter(line) {
            let n = cap[1].parse::<usize>().unwrap();
            let c = match &cap[2] {
                "r" => 0,
                "g" => 1,
                "b" => 2,
                _ => panic!("Invalid color"),
            };
            v[c] = v[c].max(n);
        }
        // println!("{}: {:?} -> {}", i, v, (v[0]<=12 && v[1]<=13 && v[2]<=14));
        part1 += (i+1) * (v[0]<=12 && v[1]<=13 && v[2]<=14) as usize;
        part2 += v[0]*v[1]*v[2];
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}