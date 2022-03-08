/*  ex01a_nan  */
fn main() {
    let x: f64 = 2.5;
    let y: f64 = f64::NAN;
    println!("x == x: {:6} x != y: {}", x==x, x!=x);
    println!("y == y: {:6} y != y: {}", y==y, y!=y);
}
// x == x: true   x != y: false
// y == y: false  y != y: true




//********************************//


/*  ex02f_partialeq */
// https://doc.rust-lang.org/std/cmp/trait.PartialEq.html

pub trait PartialEq<Rhs = Self>
where Rhs: ?Sized, {

    fn eq(&self, other: &Rhs) -> bool;

    fn ne(&self, other: &Rhs) -> bool { ... }
}



//********************************//


/*  ex03a_eq  */
fn main() {
    let x: f64 = 2.5;
    let y: f64 = f64::NAN;
    println!("x == x: {}", x.eq(&x));
    println!("y == y: {}", y.eq(&y));
    println!("{} is NaN: {}", y, y.is_nan());
}
// x == x: true
// y == y: false
// NaN is NaN: true




//********************************//


/*  ex04f_eq */
// https://doc.rust-lang.org/std/cmp/trait.Eq.html

pub trait Eq: PartialEq<Self> { }

/*

Trait Eq är kodmässigt samma som PartialEq.

Skillnaden mellan dem är att i Eq krävs att varje värde
är lika med sig självt. Detta kan inte kompilatorn
kontrollera, så den får helt enkelt lita på om koden
säger Eq i stället för PartialEq.

*/



//********************************//


/*  ex05a_ordning  */
fn main() {
    let x: f64 = 2.5;
    let y: f64 = f64::NAN;
    println!("x < y: {}", x < y);
    println!("y < x: {}", y < x);
    println!("x == y: {}", x == y);
}
// x < y: false
// y < x: false
// x == y: false



//********************************//


/*  ex06f_partialord */
// https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html

pub trait PartialOrd<Rhs = Self>: PartialEq<Rhs>
where Rhs: ?Sized, {

    fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;

    fn lt(&self, other: &Rhs) -> bool { ... }
    fn le(&self, other: &Rhs) -> bool { ... }
    fn gt(&self, other: &Rhs) -> bool { ... }
    fn ge(&self, other: &Rhs) -> bool { ... }
}



//********************************//


/*  ex07a_partial_cmp */
fn main() {
    let x: f64 = 1.7;
    let y: f64 = 2.5;
    let z: f64 = f64::NAN;
    println!("x < y: {:?}", x.partial_cmp(&y));
    println!("y < z: {:?}", y.partial_cmp(&z));
}
// x < y: Some(Less)
// y < z: None



//********************************//


/*  ex08a_sort_f64 */
fn main() {
    let mut lista = [12.3, 18.9, 33.4, 27.0_f64];
    if lista.iter().find(|&x| x.is_nan()) == None {
	lista.sort_by(
	    |x, y| x.partial_cmp(y).unwrap()
	);
	println!("{:?}", lista);
    }
}
// [12.3, 18.9, 27.0, 33.4]



//********************************//


/*  ex09a_kortlek  */
#[derive(PartialEq)]
enum Färg {
    Klöver(u8),
    Ruter(u8),
    Hjärter(u8),
    Spader(u8),
}

use std::cmp::Ordering;
use Färg::*;

impl PartialOrd for Färg {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
	    (Klöver(m), Klöver(n)) => Some(m.cmp(n)),
	    (Ruter(m), Ruter(n)) => Some(m.cmp(n)),
	    (Hjärter(m), Hjärter(n)) => Some(m.cmp(n)),
	    (Spader(m), Spader(n)) => Some(m.cmp(n)),
	    _ => None,
	}
    }
}

fn main() {
    let s = Hjärter(5);
    let t = Hjärter(13);
    println!("{:?}", s.partial_cmp(&t));
    println!("{:?}", s.partial_cmp(&Klöver(4)));
}
// Some(Less)
// None




//********************************//


/*  ex10f_ord */
// https://doc.rust-lang.org/std/cmp/trait.Ord.html

pub trait Ord: Eq + PartialOrd<Self> {
    fn cmp(&self, other: &Self) -> Ordering;

    fn max(self, other: Self) -> Self { ... }
    fn min(self, other: Self) -> Self { ... }
    fn clamp(self, min: Self, max: Self) -> Self { ... }
}



//********************************//


/*  ex11a_struct  */
struct Punkt {
    x: f64,
    y: f64,
}

fn main() {
    let p = Punkt { x: 4.0, y: -1.0 };
    println!("Punkt: ({}, {})", p.x, p.y);
}
// Punkt: (4, -1)



//********************************//


/*  ex12a_derive  */
#[derive(PartialEq, Debug)]
struct Punkt {
    x: f64,
    y: f64,
}

fn main() {
    let p = Punkt { x: 4.0, y: -1.0 };
    println!("{:?} Lika: {}", p, p == p);
}
// Punkt { x: 4.0, y: -1.0 } Lika: true



//********************************//


/*  ex13e_move  */

#[derive(PartialEq, Debug)]
struct Punkt { x: f64, y: f64 }

fn main() {
    let p = Punkt { x: 4.0, y: -1.0 };
    let p2 = p;
    println!("Lika: {}", p == p2);
}
// error[E0382]: borrow of moved value

/*  ex14f_copy  */
// https://doc.rust-lang.org/std/marker/trait.Copy.html

pub trait Copy: Clone { }

/*  ex15f_clone  */
// https://doc.rust-lang.org/std/clone/trait.Clone.html

pub trait Clone {
    fn clone(&self) -> Self;

    fn clone_from(&mut self, source: &Self) { ... }
}

/*  ex16f_kopia  */
#[derive(PartialEq, Debug, Copy, Clone)]
struct Punkt { x: f64, y: f64 }

fn main() {
    let p = Punkt { x: 4.0, y: -1.0 };
    let p2 = p;
    println!("Lika: {}", p == p2);
    println!("Kopia: {:?}", p.clone());
}
// Lika: true
// Kopia: Punkt { x: 4.0, y: -1.0 }

/*  ex17f_add  */
// https://doc.rust-lang.org/std/ops/trait.Add.html

trait Add<Rhs = Self> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}



//********************************//


/*  ex18e_addera  */

pub fn addera<T>(x: T, y: T) -> T {
    x + y
}

fn main() {
    let a: isize = 5;
    let b = 3;
    println!("{a} + {b} = {}", addera(a, b));
}
// error[E0369]: cannot add `T` to `T`


/*  ex19a_addera  */
use std::ops::Add;

pub fn addera<T: Add<Output = T>>(x: T, y: T) -> T {
    x + y
}

fn main() {
    let a: isize = 5;
    let b = 3;
    println!("{} + {} = {}", a, b, addera(a, b));
}
// 5 + 3 = 8

/*  ex20a_addera_u8  */
pub fn addera<T>(x: T, y: T) -> T
where T: std::ops::Add<Output = T>  {
    x + y
}

fn main() {
    println!("{}", addera::<u8>(31, 97));
}
// 128

/*  ex21f_display  */
// https://doc.rust-lang.org/std/fmt/trait.Display.html

pub trait Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
}
// även: to_string


/*  ex22a_skriv_ut  */
fn skriv_ut(sekvens: &[&str]) {
    for värde in sekvens {
	print!("{:?} ", värde);
    }
    println!();
}

fn main() {
    let v = vec!["Bill", "Steve", "Linus"];
    skriv_ut(&v);
}
// "Bill" "Steve" "Linus"

/*  ex23a_generisk  */
fn skriv_ut<C, T>(sekvens: C)
where C: IntoIterator<Item=T>,
      T: std::fmt::Debug
{
    for värde in sekvens {
	print!("{:?} ", värde);
    }
    println!();
}

fn main() {
    let v = vec!["Bill", "Steve", "Linus"];
    skriv_ut(&v);
}
// "Bill" "Steve" "Linus" 



//********************************//


/* Övning 501: score

Skapa en struct med namnet Score. Den ska ha
attributen name (String) och score (f64).

Se till att Score har lämpliga traits, bl.a.
PartialOrd (jämför score oberoende av name).

I main-funktionen, skapa en Vector av Score.
Sortera vektorn och skriv ut den.

*/


//********************************//


