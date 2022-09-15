use std::cmp::max;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

pub fn run() {
    let mut h = HashMap::new();
    for l in io::stdin().lock().lines() {
        println!("{:?}", f(l.unwrap().parse().unwrap(), &mut h));
    }
}

fn f(n: u32, h: &mut HashMap<u32, u32>) -> u32 {
    if n == 0 {
        return 0;
    }
    let mut r = *h.entry(n).or_default();
    if r == 0 {
        r = max(n.into(), f(n / 2, h) + f(n / 3, h) + f(n / 4, h));
        *h.entry(n).or_default() += r;
    }
    return r;
}
