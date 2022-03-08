/*  ex43e_moves  */
fn main() {
    let lista = vec![
	"Bill".to_string(), "Steve".to_string(),
	"Linus".to_string(), "Ken".to_string(),
    ];
    for namn in lista {
	println!("{}", namn);
    }
    println!("{:?}", lista);
}
// error[E0382]: borrow of moved value

