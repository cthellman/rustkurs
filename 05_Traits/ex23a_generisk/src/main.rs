/*  ex23a_generisk  */
fn skriv_ut<C, T>(sekvens: C)
where C: IntoIterator<Item=T>,
      T: std::fmt::Debug
{
    for värde in sekvens {
	print!("{:?} ", värde);
    }
    println!();
}

fn main() {
    let v = vec!["Bill", "Steve", "Linus"];
    skriv_ut(&v);
}
// "Bill" "Steve" "Linus" 





