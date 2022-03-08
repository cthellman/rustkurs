/*  ex01a_variabel  */
fn main() {
    let a;
    a = 5;
    println!("Värdet är {}", a);
}
// Värdet är 5


//********************************//


/*  ex02e_oinitialiserad  */
fn main() {
    let a;
    println!("Värdet är {}", a);
    a = 5;
}
// error[E0381]: borrow of possibly-uninitialized variable



//********************************//


/*  ex03a_datatyp  */
fn main() {
    let a: isize;
    a = 5;
    println!("Värdet är {}", a);
}
// Värdet är 5


//********************************//


/*  ex04a_initialisera  */
fn main() {
    let a: isize = 5;
    println!("Värdet är {}", a);
}
// Värdet är 5


//********************************//


/*  ex05a_literal  */
fn main() {
    let a = 5_isize;
    println!("Värdet är {}", a);
}
// Värdet är 5


//********************************//


/*  ex06a_typ_i8  */
fn main() {
    let a: i8 = 5;
    let b = 3;
    let c = a + b - 10;
    println!("Resultat: {}", c);
}
// Resultat: -2


//********************************//


/*  ex07a_overflow  */
#[allow(arithmetic_overflow)]
fn main() {
    let a: usize = 5;
    let b = 3;
    let c = a + b - 10;
    println!("Resultat: {}", c);
}
// thread 'main' panicked at 'attempt to subtract with overflow'


//********************************//


/*  ex08a_saturating  */
fn main() {
    let a = 100_u8;
    let b = a.saturating_add(200);
    println!("100 + 200 = {}", b);
}
// 100 + 200 = 255


//********************************//


/*  ex09e_immutable  */
fn main() {
    let a: usize = 5;
    a = a + 10;
    println!("Resultat: {}", a);
}
// error[E0384]: cannot assign twice to immutable variable




//********************************//


/*  ex10a_mutable  */
fn main() {
    let mut a: usize = 5;
    a = a + 10;
    println!("Resultat: {a}");
}
// Resultat: 15

/*

Detta kräver Rust 1.58 eller
nyare eftersom variabeln a är
inbäddad i formatsträngen.

*/


//********************************//


/*  ex11a_aritmetik  */
fn main() {
    let a: usize = 8;
    let b: usize = 5;
    println!("{} {} {} {}", a*b, a/b, a%b, a.pow(5));
    println!("{} {}", a+b*2, (a+b)*2);
}
// 40 1 3 32768
// 18 26


//********************************//


/*  ex12a_funktion  */
fn addera(a: isize, b: isize) -> isize {
    let c = a + b;
    c
}

fn main() {
    let a = 5;
    let b = 3;
    println!("{a} + {b} = {}", addera(a, b));
}
// 5 + 3 = 8

/*

Ett kodblock, dvs. kod inom klamrar ("måsvingar"), kan ha ett resultat i
Rust. Resultatet är det sista värdet som beräknas före den avslutande
klammern. Obs! Om man har ett semikolon före den avslutande klammern,
så räknas värdet som "tomt", vilket skrivs som det tomma parentesparet ()
i Rust.

I en funktion får man göra explicita return om man vill, men man bör
inte göra det i sista raden. T.ex. ger cargo clippy varningen

   warning: unneeded `return` statement

i så fall.

I funktionen addera hade man förstås inte behövt använda variabeln c.
Det hade räckt att returnera a+b.

*/


//********************************//


/*  ex13a_flyttal  */
fn main() {
    let x = 1.1;
    println!("Resultat: {}", x * x);
}
// Resultat: 1.2100000000000002


//********************************//


/*  ex14a_kvadratrot  */
fn main() {
    let x: f64 = 1.1;
    println!("Kvadrat: {:.3}", x * x);
    let y: f32 = 2.0;
    println!("Rot: {:.6}", y.sqrt());
}
// Kvadrat: 1.210
// Rot: 1.414214


//********************************//


/*  ex15a_fibonacci  */
fn main() {
    let mut a: usize = 0;
    let mut b = 1;
    while a < 500 {
	print!("{} ", a);
	let c = a + b;
	a = b;
	b = c;
    }
    println!();
}
// 0 1 1 2 3 5 8 13 21 34 55 89 144 233 377


//********************************//


/*  ex16a_if  */
fn main() {
    let x: f64 = 1.1;
    if x > 1.0 {
	println!("Stort tal.");
    }
}
// Stort tal.


//********************************//


/*  ex17a_ifelse  */
fn main() {
    let a = 5;
    let b = 3;
    let c = if a > b { a } else { b };
    println!("Största värdet: {}", c);
}
// Största värdet: 5


//********************************//


/*  ex18a_unit  */
fn main() {
    let a = 5;
    let b = 3;
    let c = if a > b { a; } else { b; };
    println!("Största värdet: {:?}", c);
}
// Största värdet: ()


//********************************//


/*  ex19a_block  */
fn main() {
    let a = 2;
    let b = {
	let x = 3;
	let y = 4;
	a + x * y
    };
    println!("Resultat: {}", b);
}
// Resultat: 14


//********************************//


/*  ex20a_loop  */
fn main() {
    for i in 1..5 {
	println!("{}", i);
    }
}
// 1
// 2
// 3
// 4


//********************************//


/*  ex21a_inklusive  */
fn main() {
    for i in 1..=5 {
	println!("{}", i);
    }
}
// 1
// 2
// 3
// 4
// 5


//********************************//


/*  ex22a_array  */
fn main() {
    let a: [usize; 5] = [12, 19, 33, 27, 22];
    for i in 0..5 {
	print!("{} ", a[i]);
    }
}
// 12 19 33 27 22


//********************************//


/*  ex23a_mut_array  */
fn main() {
    let mut a = [12, 19, 33, 27, 22];
    for i in 0..5 {
	a[i] = a[i] - 10;
	print!("{} ", a[i]);
    }
}
// 2 9 23 17 12 


//********************************//


/*  ex24a_match  */
fn main() {
    let tal = 1;
    let res = match tal {
	0 => "Inget",
	1 => "Ett",
	2 => "Två",
	_ => "Annat",
    };
    println!("Resultat: {}", res);
}
// Resultat: Ett


//********************************//


/*  ex25a_intervall  */
fn main() {
    let tal: usize = 42;
    match tal {
	0..=9 => {
	    println!("Ensiffrigt");
	}
	_ => println!("Inte ensiffrigt"),
    };
}
// Inte ensiffrigt


//********************************//


/*  ex26a_str  */
fn main() {
    let s1 = "Hej\thopp";
    let s2 = r"Tecknen \n ger radbrytning";
    println!("{}", s1);
    println!("{}", s2);
    println!("{:?}", b"G\xf6ran");
}
// Hej	hopp
// Tecknen \n ger radbrytning
// [71, 246, 114, 97, 110]

/*

Explicita strängar ("string literals") skrivs inom citationstecken.
Tecknet \ har specialbetydelse; \t betyder TAB-tecken, \n betyder
radmatning, \\ betyder bakvänt snedstreck osv.

Ifall man skriver ett r före citationstecknet, så tas specialbetydelsen
av \ bort.

Observera! I Rust är strängar kodade i utf-8! Källkoden ska använda utf-8
som teckenkodning (eller hålla sig till ASCII).

Om man har text kodad i någon annan teckenkodning, så måste man betrakta
det som "binärdata", vilket i Rust betyder "en följd av bytes".

Vill man skriva en explicit "binärsträng" i Rust, så ska man sätta
tecknet r före citationstecknet, och icke-ASCII-tecken skrivs med
hexadecimala koder.

Formateringskoden :? betyder att man vill göra en "debug-utskrift" av
motsvarande värde.

*/


//********************************//


/*  ex27a_string  */
fn main() {
    let namn: String = String::from("Steve");
    println!("Namn: {}", namn);
}
// Namn: Steve


//********************************//


/*  ex28a_len  */
fn main() {
    let p = String::from("Göran");
    println!("Storlek: {}", p.len());
    println!("Längd: {}", p.chars().count());
}
// Storlek: 6
// Längd: 5


//********************************//


/*  ex29a_push  */
fn main() {
    let mut namn = String::from("Steve");
    namn.push_str(" Jobs");
    println!("Namn: {}", namn);
}
// Namn: Steve Jobs


//********************************//


/*  ex30e_moved  */
fn main() {
    let namn = String::from("Steve");
    let p = namn;
    println!("Namn: {}", namn);  // Error
}
// error[E0382]: borrow of moved value: `namn`


//********************************//


/*  ex31a_move  */
fn main() {
    let namn = String::from("Steve");
    let mut p = namn;
    p.push_str(" Jobs");
    println!("Namn: {}", p);
}
// Namn: Steve Jobs


//********************************//


/*  ex32a_likhet  */
fn main() {
    let namn = String::from("Steve");
    let p = String::from("Steve");
    if namn == p { println!("Lika"); }
    if namn == "Steve" { println!("Också lika"); }
}
// Lika
// Också lika


//********************************//


/*  ex33a_clone  */
fn main() {
    let namn = String::from("Steve");
    let p = namn.clone();
    println!("Namn: {} och {}", namn, p);
}
// Namn: Steve och Steve


//********************************//


/*  ex34a_referens  */
fn main() {
    let poäng = 33;
    let referens = &poäng;
    let ny_poäng = *referens + 10;
    println!("Värdet är {} var {}", ny_poäng, poäng);
}
// Värdet är 43 var 33


//********************************//


/*  ex35a_mut_ref  */
fn main() {
    let mut poäng = 33;
    {
        let r = &mut poäng;
        *r += 10;
    }
    println!("Värdet är {}", poäng);
}
// Värdet är 43


//********************************//


/*  ex36a_by_reference  */
fn öka(tal: &mut i32) {
    *tal += 10;
}

fn main() {
    let mut poäng = 33;
    öka(&mut poäng);
    println!("Värdet är {}", poäng);
}
// Värdet är 43


//********************************//


/* Övning 201: Loop

Gör ett program som skriver ut talen
0, 1, 2, 3, ... 10 på var sin rad.

*/


//********************************//


/* Övning 202: Talsumma

Skriv funktionen talsumma som tar ett usize
som parameter och returnerar summan av alla tal
från 0 till och med det givna talet.
T.ex. ska talsumma(3) returnera 6.

När det går att kompilera och exekvera programmet,
kör enhetstesterna, t.ex. med kommandot

   cargo test

eller via menyerna i editorn.

 */



fn main() {
    println!("Summa: {}", talsumma(20));
}

// Lämna koden nedan oförändrad

#[test]
fn test_talsumma1() {
    assert_eq!(talsumma(1), 1);
}

#[test]
fn test_talsumma10() {
    assert_eq!(talsumma(10), 55);
    assert_eq!(talsumma(20), 210);
}

#[test]
fn test_talsumma0() {
    assert_eq!(talsumma(0), 0);
}


//********************************//


/* Övning 203: Dubblera

Skriv funktionen dubblera som tar en referens till
en isize som parameter och dubblerar värdet på
variabeln som refereras.

 */



fn main() {
    let mut n: isize = 7;
    dubblera(&mut n);
    println!("Värde: {}", n);
}

// Lämna koden nedan oförändrad

#[test]
fn test_dubblera1() {
    let mut i: isize = -7;
    dubblera(&mut i);
    assert_eq!(i, -14);
}

#[test]
fn test_dubblera2() {
    let mut i: isize = 0;
    dubblera(&mut i);
    assert_eq!(i, 0);
}

#[test]
fn test_dubblera3() {
    let mut i: isize = 99;
    dubblera(&mut i);
    assert_eq!(i, 198);
}


//********************************//


/* Övning 204: Fibonacci

Skriv funktionen fibomax. Den ska ta en u32 som parameter
och ska returnera det största Fibonaccitalet som är mindre
än eller lika med det givna talet.

T.ex. ska fibomax(100) returnera 89.

*/



fn main() {
    println!("Resultat: {}", fibomax(100));
}

// Lämna koden nedan oförändrad

#[test]
fn test_fibomax1() {
    assert_eq!(fibomax(100), 89);
}

#[test]
fn test_fibomax2() {
    assert_eq!(fibomax(300), 233);
}

#[test]
fn test_fibomax3() {
    assert_eq!(fibomax(0), 0);
}

#[test]
fn test_fibomax4() {
    assert_eq!(fibomax(1), 1);
}

#[test]
fn test_fibomax5() {
    assert_eq!(fibomax(2), 2);
}

#[test]
fn test_fibomax6() {
    assert_eq!(fibomax(3_000_000_000), 2971215073);
}


//********************************//


