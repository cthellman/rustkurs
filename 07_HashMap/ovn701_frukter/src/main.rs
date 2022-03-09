/* Övning 701: frukter

Skapa funktionen kilopris. Den ska ta en referens
till en HashMap::<&str, f64> som parameter.
Nycklarna är namn på frukter, och värdena är kilopris.
Funktionen ska skriva ut frukterna, en per rad, sorterade
i växande ordning efter kilopriset. T.ex.
Banan kostar 25.00 kr
Päron kostar 29.75 kr

 */


use std::collections::HashMap;

fn kilopris(f: &HashMap::<&str, f64>) {
    let mut v: Vec<_> = f.iter().collect();
    v.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());
    for (&frukt, &pris) in v {
	println!("{} kostar {:.2} kr", frukt, pris);
    }
}

fn main() {
    let frukter = HashMap::from([
	("Äpple", 34.95), ("Banan", 25.00),
	("Päron", 29.75), ("Druva", 32.50),
	("Melon", 30.00), ("Apelsin", 19.95),
    ]);
    kilopris(&frukter);
}
