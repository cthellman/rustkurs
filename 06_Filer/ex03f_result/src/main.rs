/*  ex03f_result  */

// Om res är ett Result, så
// betyder res? ungefär följande:
match res {
    Ok(n) => n,
    Err(fel) => return Err(fel)
}





