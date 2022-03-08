/*  ex20a_addera_u8  */
pub fn addera<T>(x: T, y: T) -> T
where T: std::ops::Add<Output = T>  {
    x + y
}

fn main() {
    println!("{}", addera::<u8>(31, 97));
}
// 128

