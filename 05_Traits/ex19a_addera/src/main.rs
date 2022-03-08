/*  ex19a_addera  */
use std::ops::Add;

pub fn addera<T: Add<Output = T>>(x: T, y: T) -> T {
    x + y
}

fn main() {
    let a: isize = 5;
    let b = 3;
    println!("{} + {} = {}", a, b, addera(a, b));
}
// 5 + 3 = 8

