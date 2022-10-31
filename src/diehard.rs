use std::io;
use std::io::prelude::*;

pub fn run() {
    for l in io::stdin().lock().lines().skip(1) {
        let v = l
            .unwrap()
            .split(" ")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<u16>>();

        println!("{:?}", f(v[0], v[1]));
    }
}

fn f(mut h: u16, mut a: u16) -> u16 {
    let mut r = 1;
    h = h + 3;
    a = a + 2;

    loop {
        if h > 20 && a <= 10 {
            h = h - 17;
            a = a + 7;
            r = r + 2;
        } else if h > 5 && a > 10 {
            h = h - 2;
            a = a - 8;
            r = r + 2;
        } else {
            break;
        }
    }

    return r;
}
