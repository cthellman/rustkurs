/* Övning 701: frukter

Skapa funktionen kilopris. Den ska ta en referens
till en HashMap::<&str, f64> som parameter.
Nycklarna är namn på frukter, och värdena är kilopris.
Funktionen ska skriva ut frukterna, en per rad, sorterade
i växande ordning efter kilopriset. T.ex.
Banan kostar 25.00 kr
Päron kostar 29.75 kr

 */

use std::{collections::HashMap};

fn main()
{
    let mut frukter = HashMap::<&str, f64>::new();
    frukter.insert("Banan", 25.00);
    frukter.insert("Päron", 29.75);
    frukter.insert("Äpple", 100.00);
    frukter.insert("Apelsin", 44.54);
    frukter.insert("Papaya",    2.2);
    frukter.insert("Bappelsin", 2.1);
    
    kilopris(&frukter);
    // println!("{:?}", frukter);

    let tal = 12.34;
    let tal2: isize; 
    tal2 = tal as isize;
    println!("{} {}",tal,tal2);
}

fn kilopris(frukter: &HashMap<&str,f64>) //-> HashMap<&str, f64> 
{
    // println!("{:?}", frukter);
    let mut lista:Vec<_> = frukter.iter().collect();
    lista.sort_by_key(|f| *f.1 as isize);
    for (namn, pris) in &lista {
        println!("{} kostar {} kr",&namn,&pris);
    }
    // frukter
    // frukter.iter().map(|f|f.1)
}


/*
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
*/