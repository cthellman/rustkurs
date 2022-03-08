/*  ex11a_flexible  */
fn max<'a, 'b>(v1: &'a [String], v2: &'b [String]) -> Option<&'b String> {
    if let Some(s1) = v1.iter().max_by_key(|s| s.len()) {	
	v2.iter().find(|s| s.len() >= s1.len())
    } else {
	None
    }
}

fn main() {
    let m;
    let n2: Vec<String> = vec!["Linus".to_string(), "Ken".to_string()];
    {
	let n1: Vec<String> = vec!["Bill".to_string(), "Steve".to_string()];
	m = max(&n1, &n2);
    }
    println!("Värdet är {:?}", m);
}
// Värdet är Some("Linus")

