/*  ex08a_saturating  */
fn main() {
    let a = 100_u8;
    let b = a.saturating_add(200);
    println!("100 + 200 = {}", b);
}
// 100 + 200 = 255
