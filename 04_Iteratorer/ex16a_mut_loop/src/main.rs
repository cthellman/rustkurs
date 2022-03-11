/*  ex16a_mut_loop  */
fn main() {
    let mut primtal = [2, 3, 5, 7, 11];
    for tal in &mut primtal {
	*tal += 1;
    }
    println!("Array: {:?}", primtal);   // :? prints the whole array
}
// Array: [3, 4, 6, 8, 12]


