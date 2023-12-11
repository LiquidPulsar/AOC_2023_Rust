use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");
    let mut lines = contents.lines();

    let times = lines.next().unwrap().split(' ').filter_map(|x| x.parse::<f32>().ok()).collect::<Vec<_>>();
    let dists = lines.next().unwrap().split(' ').filter_map(|x| x.parse::<f32>().ok()).collect::<Vec<_>>();
    
    /*
            for s in range(t):
            dist = (t-s) * s
            if dist>d: print(f"{s}({dist}) ",end="")
        print()
        l = (t - (t**2 - 4*d)**.5)/2
        r = (t + (t**2 - 4*d)**.5)/2
        # Roots on exact boundary, so shift by epsilon for >
        l = ceil(l + 1e-9)
        r = floor(r - 1e-9)
        print(f"{r-l+1=}")
        tot *= r-l+1
     */
    
    
    println!("Part 1: {:?}", times.iter().zip(dists.iter()).map(|(&t,&d)|
        ((t + (t*t - 4.0*d).sqrt())/2.0 - 1.0).ceil() - 
        ((t - (t*t - 4.0*d).sqrt())/2.0 + 1.0).floor() + 1.0
    ).product::<f32>());

    let mut lines = contents.lines();
    let t:f32 = lines.next().unwrap()[6..].replace(' ', "").parse().unwrap();
    let d:f32 = lines.next().unwrap()[9..].replace(' ', "").parse().unwrap();
    println!("Part 2: {:?}",
    ((t + (t*t - 4.0*d).sqrt())/2.0 - 1.0).ceil() - 
    ((t - (t*t - 4.0*d).sqrt())/2.0 + 1.0).floor() + 1.0);
}
