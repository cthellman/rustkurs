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





