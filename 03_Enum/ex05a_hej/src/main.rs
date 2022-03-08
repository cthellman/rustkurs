/*  ex05a_hej  */
fn hälsa(namn: &str) {
    println!("Hej, {}!", namn);
}

fn main() {
    hälsa("Linus");
    let s = "Ken Thompson".to_string();
    hälsa(&s);
    hälsa(&s[0..3]);
}
// Hej, Linus!
// Hej, Ken Thompson!
// Hej, Ken!
