/*  ex08a_saturating  */
fn main() {
    let a = 100_u8;
    let b = a.saturating_add(200);  // try to add 200 but only to max of what the datatype b can hold
    println!("100 + 200 = {b}");
}
// 100 + 200 = 255
