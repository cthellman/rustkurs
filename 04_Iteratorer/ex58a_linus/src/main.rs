/*  ex58a_linus  */
fn jämför(a: &&str, b: &&str) -> std::cmp::Ordering {
    if *a == "Linus" {
	std::cmp::Ordering::Less
    } else if *b == "Linus" {
	std::cmp::Ordering::Greater
    } else {
	a.cmp(&b)
    }
}

fn main() {
    let mut lista = ["Bill", "Steve", "Linus", "Ken"];
    lista.sort_by(jämför);
    println!("{:?}", lista);
}
// ["Linus", "Bill", "Ken", "Steve"]

