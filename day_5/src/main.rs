use std::{fs, vec};

type Int = i64;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");
    let mut lines = contents.lines();

    let seeds = lines.next().unwrap().split(' ').filter_map(|x| x.parse::<Int>().ok()).collect::<Vec<_>>();
    let p2ranges = seeds.chunks(2).map(|x| {
        Range::new(x[0],x[0]+x[1])
    }).collect::<Vec<_>>();

    lines.next();

    let mut steps = Vec::new();
    while let Some(_) = lines.next() {
        let mut map = RangeMap::new();
        for nxt in lines.by_ref() {
            if nxt.is_empty() {
                break;
            }
            let mut parts = nxt.split(' ');
            let d = parts.next().unwrap().parse::<Int>().unwrap();
            let s = parts.next().unwrap().parse::<Int>().unwrap();
            let r = parts.next().unwrap().parse::<Int>().unwrap();
            map.add(s,s+r,d-s);
        }
        map.ranges.sort_by(|a,b| a.0.start.cmp(&b.0.start));
        steps.push(map);
    }
    println!("Part 1: {:?}", seeds.iter()
                                .map(|x| steps.iter().fold(*x, |acc, x| x.get(acc)))
                                .min().unwrap());
    println!("Part 2: {:?}", steps.iter().fold(
                                p2ranges, 
                                |acc, x| acc.iter()
                                            .flat_map(|r| x.get_range(r.clone()))
                                            .collect::<Vec<_>>())
                                .iter()
                                .map(|x| x.start)
                                .min().unwrap());
}

#[derive(Debug,Clone)]
struct Range {
    start: Int,
    end: Int
}

impl Range {
    fn new(start: Int, end: Int) -> Self {
        assert!(start < end);
        Self {
            start,
            end,
        }
    }

    fn shift_by(&self, delta: &Int) -> Self {
        Self {
            start: self.start + delta,
            end: self.end + delta,
        }
    }

    // Returns the intersection of two ranges, and any ranges that are not in the intersection
    fn intersect(&self, other: &Range) -> (Option<Range>,Vec<Range>) {
        let mut res = Vec::new();
        
        if (self.end <= other.start) || (self.start >= other.end) {
            return (None,res);
        }

        if self.start < other.start {
            res.push(Range::new(self.start, other.start));
        }

        if self.end > other.end {
            res.push(Range::new(other.end, self.end));
        }

        let start = self.start.max(other.start);
        let end = self.end.min(other.end);


        (Some(Range::new(start,end)),res)
    }
}

#[derive(Debug)]
struct RangeMap {
    ranges: Vec<(Range,Int)>
}

impl RangeMap {
    fn new() -> Self {
        Self {
            ranges: Vec::new()
        }
    }

    fn add(&mut self, start: Int, end: Int, delta: Int) {
        self.ranges.push((Range::new(start,end),delta));
        // self.ranges.sort_by(|a,b| a.0.start.cmp(&b.0.start));
    }

    fn get(&self, i: Int) -> Int {
        for (r, d) in &self.ranges {
            if i >= r.start && i < r.end {
                return i + d;
            }
        }
        i
    }

    fn get_range(&self, range: Range) -> Vec<Range> {
        let mut res = Vec::new();
        let mut to_do = vec![range];

        'l: while let Some(r) = to_do.pop() {
            for (r2, d) in &self.ranges {
                let (intersect,not_intersect) = r.intersect(r2);
                to_do.extend(not_intersect);
                if let Some(intersect) = intersect {
                    res.push(intersect.shift_by(d));
                    continue 'l;
                }
            }
            res.push(r);
        }
        res
    }
}