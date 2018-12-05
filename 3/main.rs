extern crate regex;

use std::io;
use regex::Regex;

#[derive(Debug)]
struct Rect {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    id: usize,
}

struct Fb {
    w: usize,
    h: usize,
    b: Vec<i32>,
}

impl Fb {
    fn new(w: usize, h: usize) -> Fb {
        Fb {
            w: w,
            h: h,
            b: vec![0; w * h],
        }
    }

    fn peek_ref(&mut self, x: usize, y: usize) -> &mut i32 {
        &mut self.b[y * self.w + x]
    }
    fn peek(&self, x: usize, y: usize) -> i32 {
        self.b[y * self.w + x]
    }
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
                        h: caps["h"].parse().unwrap(),
                        id: caps["id"].parse().unwrap()});
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

fn plot(rects: &Vec<Rect>) -> Fb
{
    let sz = max_size(rects);
    let mut fb = Fb::new(sz.0, sz.1);

    for r in rects {
        for y in r.y..(r.y + r.h) {
            for x in r.x..(r.x + r.w) {
                *fb.peek_ref(x, y) += 1;
            }
        }
    }

    fb
}

fn main() {
    let rects = read_input();
    let fb = plot(&rects);
    let total = fb.b.iter().fold(0, |acc, c| acc + (*c > 1) as i32);

    println!("{}", total);

    let whole = rects.into_iter().find(|rect| {
        let mut res: bool = false;
        'outer: for y in rect.y..(rect.y + rect.h) {
            for x in rect.x..(rect.x + rect.w) {
                res = fb.peek(x, y) == 1;
                if !res {
                    break 'outer;
                }
            }
        }
        res
    });

    println!("{:?}", whole);
}
