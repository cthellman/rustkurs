/*  ex09e_missing_tick  */
fn max(v1: &[String], v2: &[String]) -> &String {
    let s1 = v1.iter().max_by_key(|s| s.len()).unwrap();
    let s2 = v2.iter().max_by_key(|s| s.len()).unwrap();
    if s1.len() > s2.len() { s1 }  else { s2 }
}
// error[E0106]: missing lifetime specifier

/*  exa_lifetime  */
fn max<'a>(v1: &'a [String], v2: &'a [String]) -> &'a String {
    let s1 = v1.iter().max_by_key(|s| s.len()).unwrap();
    let s2 = v2.iter().max_by_key(|s| s.len()).unwrap();
    if s1.len() > s2.len() { s1 }  else { s2 }
}

fn main() {
    let n1: Vec<String> = vec!["Bill".to_string(), "Steve".to_string()];
    let n2: Vec<String> = vec!["Linus".to_string(), "Ken".to_string()];
    let m = max(&n1, &n2);
    println!("Värdet är {}", *m);
}
// Värdet är Linus


