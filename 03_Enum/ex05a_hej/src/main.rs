/*  ex05a_hej  */
fn hälsa(namn: &str) {  // The &str contains - reference, size and capacity (aka fat pointer)
    println!("Hej, {}!", namn);
}

fn main() {
    hälsa("Linus");
    let s = "Kön Thompson".to_string();
    hälsa(&s);
    hälsa(&s[0..3]); // This is the size in bytes!!!
}
// Hej, Linus!
// Hej, Ken Thompson!
// Hej, Ken!
