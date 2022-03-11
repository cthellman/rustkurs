/* Övning 401: long_words

Skriv funktionen long_words. Den ska ta en &str som
parameter och den ska returnera antalet "ord" med minst
fem bokstäver i den givna strängen. (Varje följd av
icke-blanka tecken räknas som ett ord.)

 */

fn long_words(s: &str) -> usize {
    s.split_whitespace().filter(|&s|s.chars().count()>4).count()
} 


fn main() {
    let s = "Bill 12\nSteve 19\n\
	     Linus 33\nKen 27";
    println!("Antal: {}", long_words(s));
}

#[test]
fn test_emtpy() {
    assert_eq!(long_words(""), 0);
    assert_eq!(long_words(&String::new()), 0);
}

#[test]
fn test_karlsson() {
    let s = "Som ett yrväder en aprilafton\
	     med ett höganäskrus.";
    assert_eq!(long_words(s), 3);
}

#[test]
fn test_många() {
    let mut s = String::new();
    let v = ["Kalle ", "Ingrid ", "Urban "];
    for i in 1..20 {
	s.push_str(v[i%3]);
	assert_eq!(long_words(&s) as usize, i);
    }
}

#[test]
fn test_utf8() {
    let s = "De röda stugorna";
    assert_eq!(long_words(s), 1);
}


