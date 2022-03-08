/*  ex25a_intervall  */
fn main() {
    let tal: usize = 42;
    match tal {
	0..=9 => {
	    println!("Ensiffrigt");
	}
	_ => println!("Inte ensiffrigt"),
    };
}
// Inte ensiffrigt
