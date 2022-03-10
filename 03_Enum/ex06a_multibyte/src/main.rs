/*  ex06a_multibyte  */
fn main() {
    let s = "Göran";    // The ö is the problem here, g is the first byte and ö is on second and third...
    println!("{}", &s[0..2]);
}
// panic: not a char boundary
