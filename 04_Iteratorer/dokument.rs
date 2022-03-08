/*  ex01a_tuple  */
fn main() {
    let res = ("Steve", 19);
    println!("{} fick {} poäng", res.0, res.1);
}
// Steve fick 19 poäng



//********************************//


/*  ex02a_variabler  */
fn main() {
    let t = (12, 19, 33, 27);
    let (a, b, c, d) = t;
    println!("{} {} {} {} ", a, b, c, d);
}
// 12 19 33 27 



//********************************//


/*  ex03a_omkastning  */
fn main() {
    let mut t = (12, 19, 33, 27);
    t = (t.3, t.1, 4, t.2);
    println!("{:?}", t);
}
// (27, 19, 4, 33)



//********************************//


/*  ex04a_korta_tupler  */
fn main() {
    let u = ();
    let t = ("Hej",);
    println!("{:?} {}", u, t.0);
}
// () Hej



//********************************//


/*  ex05a_destruct1  */
fn main() {
    let t = ("Ken", 27);
    if let ("Ken", n) = t {
	println!("{}", n);
    }
}
// 27



//********************************//


/*  ex06a_destruct2  */
fn main() {
    let t = ("Ken", &27);
    if let ("Ken", n) = t {
	println!("{}", *n);
    }
}
// 27



//********************************//


/*  ex07a_destruct3  */
fn main() {
    let t = ("Ken", &27);
    if let ("Ken", &n) = t {
	println!("{}", n);
    }
}
// 27



//********************************//


/*  ex08a_destruct4  */
fn f(&tal: &usize) {
    println!("{}", tal);
}

fn main() {
    let n: usize = 27;
    f(&n);
}
// 27



//********************************//


/*  ex09a_array  */
fn main() {
    let lista: [usize; 5] = [2, 3, 5, 7, 11];
    println!("{} och {}", lista[0], lista[3]);
    let i = rand::random::<usize>();
    println!("{}", lista[i]); // Trolig panic
}
// 2 och 7
// panic: index out of bounds




//********************************//


/*  ex10f_Option  */

// Notorisk enum
enum Option<T> {
    Some(T),
    None,
}



//********************************//


/*  ex11a_get  */
fn main() {
    let lista: [usize; 5] = [2, 3, 5, 7, 11];
    let i = rand::random::<usize>() % 6;
    if let Some(n) = lista.get(i) {
	println!("{}", n);
    } else {
	println!("Index {} är för stort", i);
    }
}
// 5



//********************************//


/*  ex12a_while_let  */
fn main() {
    let lista: [usize; 5] = [2, 3, 5, 7, 11];
    let f = rand::random::<usize>;

    while let Some(n) = lista.get(f() % 6) {
	println!("{}", n);
    }
}
// 5
// 5
// 11




//********************************//


/*  ex13a_kopia  */
fn main() {
    let primtal = [2, 3, 5, 7, 11];
    let mut kopia = primtal;
    if primtal == kopia {
	println!("Lika!");
    }
    kopia[2] = 13;
    if primtal == kopia {
	println!("Lika???");
    }
}
// Lika!




//********************************//


/*  ex14a_index  */
fn main() {
    let mut lista = [2, 3, 5, 7, 11];
    for i in 0..lista.len() {
	lista[i] = lista[i] + 1;
    }
    println!("Array: {:?}", lista);
}
// Array: [3, 4, 6, 8, 12]




//********************************//


/*  ex15a_for_loop  */
fn main() {
    let primtal = [2, 3, 5, 7, 11];
    for tal in &primtal {
	print!("{} ", *tal);
    }
}
// 2 3 5 7 11 



//********************************//


/*  ex16a_mut_loop  */
fn main() {
    let mut primtal = [2, 3, 5, 7, 11];
    for tal in &mut primtal {
	*tal = *tal + 1;
    }
    println!("Array: {:?}", primtal);
}
// Array: [3, 4, 6, 8, 12]




//********************************//


/*  ex17a_copy_loop  */
fn main() {
    let primtal = [2, 3, 5, 7, 11];
    for &tal in &primtal {
	print!("{} ", tal);
    }
}
// 2 3 5 7 11 




//********************************//


/*  ex18a_sekvens  */
fn visa(lista: &[usize]) {
    println!("Sekvens: {:?}", lista);
}

fn main() {
    let primtal = [2, 3, 5, 7, 11];
    visa(&primtal);
    visa(&primtal[2..]);
}
// Sekvens: [2, 3, 5, 7, 11]
// Sekvens: [5, 7, 11]




//********************************//


/*  ex19a_tuple_loop  */
fn main() {
    let scores = [("Bill", 12), ("Steve", 19),
		  ("Linus", 33), ("Ken", 27)];
    for (namn, poäng) in scores {
	println!("{} fick {} poäng.", namn, poäng);
    }
}
// Bill fick 12 poäng.
// Steve fick 19 poäng.
// Linus fick 33 poäng.
// Ken fick 27 poäng.




//********************************//


/*  ex20a_split  */
fn main() {
    let s = "Ett två tre fyra.";
    for ord in s.split_whitespace() {
	println!("Ord: {}", ord);
    }
}
// Ord: Ett
// Ord: två
// Ord: tre
// Ord: fyra.

/*

Metoden split_whitespace "delar upp" en &str i icke-blanka delsträngar.
Men hur? Vad returnerar den? En vektor med kopior av delsträngarna?

Nej, den returnerar en iterator. En iterator är ett objekt på vilket
man kan anropa metoden next() om och om igen för att få nya värden,
tills det inte finns några värden kvar.

Mer precist, next() returnerar värden inklädda i en Option, dvs. antingen
Some(Item) eller None, där Item är värdenas datatyp.

Ännu mer precist, en iterator är ett objekt av en typ som har Trait Iterator.

*/



//********************************//


/*  ex21f_trait_iterator  */

// https://doc.rust-lang.org/std/iter/trait.Iterator.html

trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;

    fn count(self) -> usize { ... }
    // ...
}




//********************************//


/*  ex22a_next  */
fn main() {
    let s = "Ett två tre fyra.";
    let mut it = s.split_whitespace();
    loop {
	if let Some(ord) = it.next() {
	    println!("Ord: {}", ord);
	} else {
	    break;
	}
    }
}
// Ord: Ett
// Ord: två
// Ord: tre
// Ord: fyra.


//********************************//


/*  ex23a_while_let  */
fn main() {
    let s = "Ett två tre fyra.";
    let mut it = s.split_whitespace();
    while let Some(ord) = it.next() {
	println!("Ord: {}", ord);
    }
}
// Ord: Ett
// Ord: två
// Ord: tre
// Ord: fyra.


//********************************//


/*  ex24a_while_let  */
fn main() {
    let s = "Bill 12\nSteve 19\n\
	     Linus 33\nKen 27\n";
    let mut it = s.split_whitespace();
    while let Some(namn) = it.next() {
	if let Some(p) = it.next() {
	    print!("{} fick {}. ", namn, p);
	}
    }
}
// Bill fick 12. Steve fick 19. Linus fick 33. Ken fick 27.



//********************************//


/*  ex25a_iterabel  */
fn main() {
    let primtal = [2, 3, 5, 7, 11];
    for tal in primtal {
	print!("{} ", tal);
    }
}
// 2 3 5 7 11

/*

Vi såg förut att man kan loopa över en iterator. Men man kan också loopa över
andra saker, t.ex. över arrayer eller vektorer.

Vad som händer när man loopar över ett objekt är att Rust först "hämtar ut"
en iterator från objektet, och sedan anropas next() på iteratorn tills den
returnerar None.

En iterabel är något man kan loopa över. Mer precist är det ett objekt på vilket
man kan anropa into_iter() och få en iterator i retur.
Ännu mer precist är det ett objekt av en typ som har Trait IntoIterator.

Om man anropar into_iter() på något som redan är en iterator, så returneras
iteratorn själv. Dvs. alla iteratorer är iterabler som man kan loopa över.
Däremot finns iterabler (t.ex. en vektor) som inte är iteratorer.

*/



//********************************//


/*  ex26a_for_howto  */
fn main() {
    let primtal = [2, 3, 5, 7, 11];
    // Så här gör en for-loop:
    let mut it = primtal.into_iter();
    while let Some(tal) = it.next() {
	print!("{} ", tal);
    }
}
// 2 3 5 7 11 



//********************************//


/*  ex27a_iter  */
fn main() {
    let primtal = [2, 3, 5, 7, 11];
    let mut it = primtal.iter();
    while let Some(tal_ref) = it.next() {
	print!("{} ", *tal_ref);
    }
}
// 2 3 5 7 11 



//********************************//


/*  ex28a_iter_mut  */
fn main() {
    let mut primtal = [2, 3, 5, 7, 11];
    let mut it = primtal.iter_mut();
    while let Some(tal_ref) = it.next() {
	*tal_ref += 1;
    }
    println!("{:?} ", primtal);
}
// [3, 4, 6, 8, 12] 


//********************************//


/*  ex29a_sum  */
fn main() {
    let primtal = [2, 3, 5, 7, 11];
    let n: usize = primtal.iter().sum();
    println!("{} ", n);
}
// 28



//********************************//


/*  ex30a_max  */
fn main() {
    let lista = vec![12, 19, 33, 27];
    if let Some(&n) = lista.iter().max() {    
	println!("Störst: {}", n);
    }
}
// Störst: 33



//********************************//


/*  ex31a_max_by_key  */
fn main() {
    let lista = vec![("Bill", 12),
	("Steve", 19), ("Linus", 33),
	("Ken", 27)];
    if let Some(tref) = lista.iter()
	.max_by_key(|t| t.1) {
	println!("Vinnare: {:?}", tref);
    }
}
// Vinnare: ("Linus", 33)



//********************************//


/*  ex32a_str_iters  */
fn main() {
    let namn = "Göran";
    for c in namn.chars() {
	print!("{} ", c);
    }
    for c in namn.bytes() {
	print!("{} ", c);
    }
}
// G ö r a n 71 195 182 114 97 110




//********************************//


/*  ex33a_vector  */
fn main() {
    let mut lista: Vec<usize> = Vec::new();
    lista.push(2); lista.push(3);
    lista.push(5); lista.push(7);
    println!("Lista: {:?}", lista);
    if let Some(n) = lista.pop() {
	println!("Tog bort {}", n);
    }
}
// Lista: [2, 3, 5, 7]
// Tog bort 7



//********************************//


/*  ex34a_vector_iter  */
fn main() {
    let lista = vec![2, 3, 5, 7, 11];
    let n: usize = lista.iter().sum();
    println!("Res: {}", n);
}
// Res: 28


//********************************//


/*  ex35a_vector_slice  */
fn main() {
    let lista = vec![2, 3, 5, 7, 11, 13];
    println!("Summa: {}", summa(&lista[1..5]));
    println!("Summa: {}", summa(&lista[2..]));
}

fn summa(sekvens: &[usize]) -> usize {
    sekvens.iter().sum()
}
// Summa: 26
// Summa: 36




//********************************//


/*  ex36a_collect  */
fn main() {
    let v: Vec<usize> = (1..11).collect();    
    println!("{:?}", v);
}
// [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]




//********************************//


/*  ex37a_closure  */
fn main() {
    let f = |a, b| 2 * a - b;
    println!("{}", f(3, 5));
}
// 1



//********************************//


/*  ex38a_map  */
fn main() {
    let lista = vec![2_usize, 3, 5, 7, 11];
    let v: Vec<_>;
    v = lista.iter().map(|n| *n+3).collect();
    println!("Lista: {:?}", v);
}
// Lista: [5, 6, 8, 10, 14]



//********************************//


/*  ex39a_skip  */
fn main() {
    let lista = vec![2, 3, 5, 7, 11];
    let n: usize = lista.iter().skip(2).sum();
    println!("Summa: {}", n);
}
// Summa: 23




//********************************//


/*  ex40a_siffror  */
fn main() {
    let tal = "7156";
    let n: u8 = tal.bytes().map(|b| b-48).sum();
    println!("Siffersumma: {}", n);
}
// Siffersumma: 19



//********************************//


/*  ex41a_fakultet  */
fn main() {
    let n = (1..=7).product::<usize>();
    println!("7! = {}", n);
}
// 7! = 5040




//********************************//


/*  ex42a_strings  */
fn main() {
    let lista = vec![
	"Bill".to_string(), "Steve".to_string(),
	"Linus".to_string(), "Ken".to_string(),
    ];
    println!("{:?}", lista);
}
// ["Bill", "Steve", "Linus", "Ken"]



//********************************//


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



//********************************//


/*  ex44a_for_ref  */
fn main() {
    let lista = vec![
	"Bill".to_string(), "Steve".to_string(),
	"Linus".to_string(), "Ken".to_string(),
    ];
    for namn_ref in &lista {
	print!("{} ", *namn_ref);
    }
    println!("{:?}", lista);
}
// Bill Steve Linus Ken ["Bill", "Steve", "Linus", "Ken"]



//********************************//


/*  ex45a_for  */
fn main() {
    let lista = vec![
	"Bill".to_string(), "Steve".to_string(),
	"Linus".to_string(), "Ken".to_string(),
    ];
    for namn in &lista {
	print!("{} ", namn);
    }
}
// Bill Steve Linus Ken



//********************************//


/*  ex46a_tot_len  */
fn main() {
    let lista = vec![
	"Bill".to_string(), "Steve".to_string(),
	"Linus".to_string(), "Ken".to_string(),
    ];
    let n: usize = lista.iter().map(|s| s.len()).sum();
    println!("Resultat: {}", n);
}
// Resultat: 17



//********************************//


/*  ex47a_find  */
fn main() {
    let lista = vec![
	"Bill".to_string(), "Steve".to_string(),
	"Linus".to_string(), "Ken".to_string(),
    ];
    if let Some(namn_ref) = lista.iter()
	.find(|s| (**s).len() == 3) {
	println!("Namn: {}", *namn_ref);
    }
}
// Namn: Ken



//********************************//


/*  ex48a_iter_mut  */
fn main() {
    let mut lista = vec![
	"Bill".to_string(), "Steve".to_string(),
	"Linus".to_string(), "Ken".to_string(),
    ];
    if let Some(namn) = lista.iter_mut()
	.find(|s| s.len() == 3) {
	namn.push_str("ny");
	println!("Namn: {}", namn);
    }
}
// Namn: Kenny



//********************************//


/*  ex49a_filter  */
fn main() {
    let s = "Bill 12\nLinus 33";
    let n = s.chars().
	filter(|&c| match c {
	'0'..='9' => true,
	_ => false,
    }).count();
    println!("{} siffror.", n);
}
// 4 siffror.



//********************************//


/*  ex50a_sort  */
fn main() {
    let mut lista = [12_usize, 19, 33, 27];
    lista.sort();
    println!("{:?}", lista);
}
// [12, 19, 27, 33]



//********************************//


/*  ex51a_sort_key  */
fn slutsiffra(n: &usize) -> usize {
    *n % 10
}

fn main() {
    let mut lista = [12_usize, 19, 33, 27];
    lista.sort_by_key(slutsiffra);
    println!("{:?}", lista);
}
// [12, 33, 27, 19]




//********************************//


/*  ex52a_closure  */
fn main() {
    let mut lista = [12_usize, 19, 22, 33, 27, 13];
    lista.sort_by_key(|n| *n % 10);
    println!("{:?}", lista);
}
// [12, 22, 33, 13, 27, 19]



//********************************//


/*  ex53a_tuples  */
fn main() {
    let mut v = [(3,1), (0,7), (3,2),
		 (2,5), (1,0), (2,0)];
    v.sort();
    println!("{:?} ", v);
}
// [(0, 7), (1, 0), (2, 0), (2, 5), (3, 1), (3, 2)] 



//********************************//


/*  ex54a_secondary  */
fn main() {
    let mut lista = [12_usize, 19, 22, 33, 27, 13];
    lista.sort_by_key(|n| (*n % 10, *n) );
    println!("{:?}", lista);
}
// [12, 22, 13, 33, 27, 19]




//********************************//


/*  ex55f_Ordering  */

// https://doc.rust-lang.org/std/cmp/enum.Ordering.html

pub enum Ordering {
    Less,
    Equal,
    Greater,
}



//********************************//


/*  ex56a_compare  */
fn main() {
    print!("{:?} ", 2.cmp(&5));
    print!("{:?} ", (12/3).cmp(&4));
    print!("{:?} ", 19.cmp(&-3));
    println!("{:?}", "Ulla".cmp(&"Ylva"));
}
// Less Equal Greater Less



//********************************//


/*  ex57a_jmf  */
fn jmf(a: &usize, b: &usize) -> std::cmp::Ordering {
    if a > b { std::cmp::Ordering::Greater }
    else if a == b { std::cmp::Ordering::Equal }
    else { std::cmp::Ordering::Less }
}

fn main() {
    let mut a = [12, 19, 33, 27];
    a.sort_by(jmf);
    println!("{:?}", a);
}
// [12, 19, 27, 33]



//********************************//


/*  ex58a_linus  */
fn jämför(a: &&str, b: &&str) -> std::cmp::Ordering {
    if *a == "Linus" {
	std::cmp::Ordering::Less
    } else if *b == "Linus" {
	std::cmp::Ordering::Greater
    } else {
	a.cmp(&b)
    }
}

fn main() {
    let mut lista = ["Bill", "Steve", "Linus", "Ken"];
    lista.sort_by(jämför);
    println!("{:?}", lista);
}
// ["Linus", "Bill", "Ken", "Steve"]



//********************************//


/*  ex59a_sort_by  */
fn main() {
    let mut lista = [12_usize, 19, 22, 33, 27, 13];
    lista.sort_by(|a, b| a.cmp(b) );
    println!("{:?}", lista);
}
// [12, 13, 19, 22, 27, 33]



//********************************//


/*  ex60a_sort_by2  */
fn sss(lista: &mut [usize]) {
    lista.sort_by(|a, b|
	(a%10).cmp(&(b%10)).then(a.cmp(b))
    );
}

fn main() {
    let mut lista = [12, 19, 22, 33, 27, 13];
    sss(&mut lista);
    println!("{:?}", lista);
}
// [12, 22, 13, 33, 27, 19]



//********************************//


/*  ex61a_sort_strings  */
fn main() {
    let mut lista = vec![
	"Bill".to_string(), "Steve".to_string(),
	"Linus".to_string(), "Ken".to_string(),
    ];
    lista.sort_by(|a, b|
	b.len().cmp(&b.len()).then(a.cmp(b))
    );
    println!("Resultat: {:?}", lista);
}
// Resultat: ["Bill", "Ken", "Linus", "Steve"]


//********************************//


/* Övning 401: long_words

Skriv funktionen long_words. Den ska ta en &str som
parameter och den ska returnera antalet "ord" med minst
fem bokstäver i den givna strängen. (Varje följd av
icke-blanka tecken räknas som ett ord.)

 */



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




//********************************//


/* Övning 402: siffersumma

Skriv funktionen sss. Den ska ta en sekvens av u32 som
parameter och ska sortera sekvensen i växande ordning med
avseende på talens siffersumma.

T.ex. ska [12, 19, 33, 27] sorteras som [12, 33, 27, 19]
eftersom siffersummorna är 1+2, 3+3, 2+7 respektive 1+9.

Extrauppgift: Tal som har samma siffersumma ska sorteras
i växande ordning med avseende på talet självt.

T.ex. ska [1234, 56, 28, 82] sorteras som
[28, 82, 1234, 56].


*/


fn main() {
    let mut lista = [1234, 56, 28, 82];
    sss(&mut lista);
    println!("Lista: {:?}", lista);
}

#[test]
fn test_array() {
    let mut lista = [12, 19, 33, 27];
    sss(&mut lista);
    assert_eq!(lista, [12, 33, 27, 19]);
}

#[test]
fn test_vektor() {
    let mut v = vec![9, 0, 88, 345, 15, 124];
    sss(&mut v);
    assert_eq!(v, [0, 15, 124, 9, 345, 88]);
}

#[test]
fn test_tom_lista() {
    let mut lista = [];
    sss(&mut lista);
    assert_eq!(lista, []);
}

#[test]
fn test_lika() {
    let mut lista = [1234, 56, 28, 82];
    sss(&mut lista);
    assert_eq!(lista, [28, 82, 1234, 56]);
}


//********************************//


