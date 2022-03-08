/*  ex30e_moved  */
fn main() {
    let namn = String::from("Steve");
    let p = namn;
    println!("Namn: {}", namn);  // Error
}
// error[E0382]: borrow of moved value: `namn`
