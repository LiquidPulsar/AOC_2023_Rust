use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let mut part1 = 0;
    let mut part2 = 0;
    for line in contents.lines() {
        let l = line.replace("one", "o1e")
                            .replace("two", "t2")
                            .replace("three", "t3e")
                            .replace("four", "4")
                            .replace("five", "5e")
                            .replace("six", "6")
                            .replace("seven", "7n")
                            .replace("eight", "e8")
                            .replace("nine", "9")
                            .replace("zero", "0");
        let part_1_vec:Vec<u32> = line.bytes().filter(|x| x.is_ascii_digit()).map(|x| (x - b'0') as u32).collect();
        let part_2_vec:Vec<u32> = l.as_str().bytes().filter(|x| x.is_ascii_digit()).map(|x| (x - b'0') as u32).collect();

        part1 += 10*part_1_vec[0] + part_1_vec[part_1_vec.len() - 1];
        part2 += 10*part_2_vec[0] + part_2_vec[part_2_vec.len() - 1];
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}