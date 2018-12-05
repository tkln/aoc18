extern crate regex;

use std::io;
use regex::Regex;

#[derive(Debug)]
struct Rect {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

fn read_input() -> Vec<Rect> {
    let mut rects: Vec<Rect> = Vec::new();
    let re = Regex::new(r"(?P<id>\d+) @ (?P<x>\d+),(?P<y>\d+): (?P<w>\d+)x(?P<h>\d+)").unwrap();
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        if line.len() == 0 {
            break
        }
        let caps = re.captures(&line).unwrap();
        rects.push(Rect{x: caps["x"].parse().unwrap(),
                        y: caps["y"].parse().unwrap(),
                        w: caps["w"].parse().unwrap(),
                        h: caps["h"].parse().unwrap()});
    }
    rects
}

fn max_size(rects: &Vec<Rect>) -> (usize, usize)
{
    let mut w = 0;
    let mut h = 0;

    for r in rects {
        w = std::cmp::max(w, r.x + r.w);
        h = std::cmp::max(h, r.y + r.h);
    }
    (w, h)
}

fn plot(rects: &Vec<Rect>) -> Vec<i32>
{
    let sz = max_size(rects);
    let w = sz.0 as usize;
    let h = sz.1 as usize;
    let mut fb = vec![0; w * h];

    for r in rects {
        for y in r.y..(r.y + r.h) {
            for x in r.x..(r.x + r.w) {
                fb[y * w + x] += 1;
            }
        }
    }

    fb
}

fn main() {
    let rects = read_input();
    let fb = plot(&rects);
    let total = fb.iter().fold(0, |acc, c| acc + (*c > 1) as i32);

    println!("{}", total);
    println!("{}", rects.len());
}
