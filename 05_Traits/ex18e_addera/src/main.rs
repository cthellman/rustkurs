/*  ex18e_addera  */

pub fn addera<T>(x: T, y: T) -> T {
    x + y
}

fn main() {
    let a: isize = 5;
    let b = 3;
    println!("{a} + {b} = {}", addera(a, b));
}
// error[E0369]: cannot add `T` to `T`


