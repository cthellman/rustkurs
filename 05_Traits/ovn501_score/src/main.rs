/* Övning 501: score

Skapa en struct med namnet Score. Den ska ha
attributen name (String) och score (f64).

Se till att Score har lämpliga traits, bl.a.
PartialOrd (jämför score oberoende av name).

I main-funktionen, skapa en Vector av Score.
Sortera vektorn och skriv ut den.

*/
#[derive(PartialEq, Debug, Clone)]
struct Score
{
    name: String,
    score: f64,
}

use std::cmp::Ordering;

impl PartialOrd for Score {
    fn partial_cmp(&self, other:&Self) -> Option<Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

fn main() {
    let mut scores: Vec<Score> = Vec::new();
    let score1 = Score { name:"Tomas".to_string(), score:10.0};
    let score2 = Score { name:"Bengt".to_string(), score:6.0};
    let score3 = Score { name:"Arne".to_string(), score:7.0};
    scores.push(score1);
    scores.push(score2);
    scores.push(score3);

    scores.sort_by(|p1, p2|p1.partial_cmp(&p2).unwrap());
    println!("scores = {:?}",scores);

}




