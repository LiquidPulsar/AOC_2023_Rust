use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let mut nums = Vec::new();
    for line in contents.lines() {
        nums.push(line_score(line).expect("Could not parse line"));
    }

    let mut counts = nums.iter().map(|_| 1).collect::<Vec<usize>>();

    let mut tot = 0;
    for i in 0..counts.len() {
        let n = counts[i];
        tot += n;
        for j in 0..nums[i] {
            counts[i+1+j] += n;
        }
    }

    println!("Part 1: {}", nums.iter().map(|n| 2usize.pow(*n as u32)/2).sum::<usize>());
    println!("Part 2: {}", tot);
}

fn line_score(line: &str) -> Option<usize> {
    let line = line.split(':').nth(1)?;

    let (l,r) = line.split_at(line.find('|')?);

    let l:HashSet<usize> = HashSet::from_iter(l.split(' ').filter_map(|x| x.parse().ok()));
    let r:HashSet<usize> = HashSet::from_iter(r.split(' ').filter_map(|x| x.parse().ok()));

    Some(l.intersection(&r).count())
}
