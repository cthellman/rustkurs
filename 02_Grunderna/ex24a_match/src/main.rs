/*  ex24a_match  */
fn main() {
    let tal = 1;
    let res = match tal {
	0 => "Inget",
	1 => "Ett",
	2 => "Två",
	_ => "Annat",
    };
    println!("Resultat: {}", res);
}
// Resultat: Ett
