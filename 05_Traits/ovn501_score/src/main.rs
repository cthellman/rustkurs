/* Övning 501: score

Skapa en struct med namnet Score. Den ska ha
attributen name (String) och score (f64).

Se till att Score har lämpliga traits, bl.a.
PartialOrd (jämför score oberoende av name).

I main-funktionen, skapa en Vector av Score.
Sortera vektorn och skriv ut den.

*/


#[derive(PartialEq, Debug, Clone)]
struct Score {
    name: String,
    score: f64,
}

use std::cmp::Ordering;

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
	self.score.partial_cmp(&other.score)
    }
}

fn main() {
    let mut v: Vec<Score> = Vec::new();
    let p1 = Score { name: "Bill".to_string(),
		     score: 12.0 };
    v.push(p1);
    let p2 = Score { name: "Steve".to_string(),
		     score: 19.0 };
    v.push(p2);
    let p3 = Score { name: "Linus".to_string(),
		     score: 33.0 };
    v.push(p3);
    let p4 = Score { name: "Steve".to_string(),
		     score: 27.0 };
    v.push(p4);
    v.sort_by(
	|p1, p2| p1.partial_cmp(&p2).unwrap()
    );
    println!("v = {:?}", v);
}


