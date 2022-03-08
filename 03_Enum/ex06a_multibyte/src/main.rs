/*  ex06a_multibyte  */
fn main() {
    let s = "Göran";
    println!("{}", &s[0..2]);
}
// panic: not a char boundary
