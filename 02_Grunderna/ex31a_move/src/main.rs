/*  ex31a_move  */
fn main() {
    let namn = String::from("Steve");
    let mut p = namn;
    p.push_str(" Jobs");
    println!("Namn: {}", p);
}
// Namn: Steve Jobs
