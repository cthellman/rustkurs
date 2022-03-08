/*  ex20a_split  */
fn main() {
    let s = "Ett två tre fyra.";
    for ord in s.split_whitespace() {
	println!("Ord: {}", ord);
    }
}
// Ord: Ett
// Ord: två
// Ord: tre
// Ord: fyra.

/*

Metoden split_whitespace "delar upp" en &str i icke-blanka delsträngar.
Men hur? Vad returnerar den? En vektor med kopior av delsträngarna?

Nej, den returnerar en iterator. En iterator är ett objekt på vilket
man kan anropa metoden next() om och om igen för att få nya värden,
tills det inte finns några värden kvar.

Mer precist, next() returnerar värden inklädda i en Option, dvs. antingen
Some(Item) eller None, där Item är värdenas datatyp.

Ännu mer precist, en iterator är ett objekt av en typ som har Trait Iterator.

*/

