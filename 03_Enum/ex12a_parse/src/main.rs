/*  ex12a_parse  */
fn main() {
    let res: Result<usize, _> = "128".parse();
    match res {
	Ok(n) => println!("Talet är {}", n),
	Err(fel) => println!("Ej tal: {}.", fel),
    }

    let str_res: Result<String, _> = "kalle".parse();
    match str_res {
	Ok(n) => println!("Strängen är {}", n),
	Err(fel) => println!("Ej tal: {}.", fel),
    }
}
// Talet är 128
// Strängen är kalle
