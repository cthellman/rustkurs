/*  ex40a_siffror  */
fn main() {
    let tal = "7156";
    let n: u8 = tal.bytes().map(|b| b-48).sum();
    println!("Siffersumma: {}", n);
}
// Siffersumma: 19

