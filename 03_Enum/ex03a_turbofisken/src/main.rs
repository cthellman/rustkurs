/*  ex03a_turbofisken  */
fn main() {
    println!("Tal: {}", rand::random::<u8>());  // random is generic and need to be told what datatype to randomize, in this example a <u8> - turbofish
}
// Tal: 36

