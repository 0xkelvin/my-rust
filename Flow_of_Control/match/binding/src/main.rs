// Indirectly accessing a variable makes it impossible to branch and use that variable without
// rebinding, match provides the @ sigil for binding values to names
//
//  A function age which returns a u32
fn age() -> u32 {
    15
}

fn some_number() -> Option<u32> {
    Some(42)
}

fn main() {
    println!("Tell me what type of person you are");

    match age() {
        0 => println!("I haven't celebrated "),
        // Could match 1 ..=12 directly but then what age
        // would the child be ?
        // Could match n and use an if guard, but would
        // not contribute exhaustiveness checks
        // Instead, bind to n for the sequence of 1 ..= 12.
        // Now the age can be reported
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        n @ 13..=19 => eprintln!("I'm a teen of age {:?}", n),
        // Nothing bound, return the result
        n => println!("I'm an old person of age {:?}", n),
    }

    match some_number() {
        // Got `Some` variant, match if its value, bound to `n`,
        // is equal to 42.
        // Could also use `Some(42)` and print `"The Awnser: 42!"`
        // but that would require changing `42` in 2 spots should
        // you ever wish to change it.
        // Could also use `Some(n) if n == 42` and print `"The Awnser: {n}!"`
        // but that would not contribute to exhaustiveness checks.
        // (Although in this case that would not matter since
        // the next arm is a "catch-all" pattern)
        Some(n @ 42) => println!("The Answer: {}!", n),
        // Match any other number
        Some(n) => println!("Not interesting ... {}", n),
        // Match anything else
        _ => (),
    }
}
