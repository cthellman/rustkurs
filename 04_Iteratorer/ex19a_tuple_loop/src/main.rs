/*  ex19a_tuple_loop  */
fn main() {
    let scores = [("Bill", 12), ("Steve", 19),
		  ("Linus", 33), ("Ken", 27)];
    for (namn, poäng) in scores {
	println!("{} fick {} poäng.", namn, poäng);
    }
}
// Bill fick 12 poäng.
// Steve fick 19 poäng.
// Linus fick 33 poäng.
// Ken fick 27 poäng.


