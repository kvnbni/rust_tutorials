//Primitive str= Immutable fixed length string somewhere in memory
//String = Growable, heap allocated data structure - use when you need to modify or own string data

pub fn run() {
    //Primitive string
    let hello = "Hello";

    //Growable string
    let mut world = String::from("W");

    //Pushing a char to a string
    world.push('o');

    //Pushing another string to a string
    world.push_str("rld");

    println!("{} {}", hello, world);

    //Get the length
    println!("{}", hello.len());

    let name = String::from("Kevin");

    //Is empty
    println!("Is the name variable empty {}", name.is_empty());

    //Contains substring
    println!("Contains substring 'Kev' {}", name.contains("Kev"));

    //Replace
    println!("Replace Kev with 'Nev' {}", name.replace("Kev", "Nev"));

    let phrase = String::from("Hello from the other side!!");

    //Loop through string by whitespace
    for word in phrase.split_whitespace() {
        println!("{}", word);
    }

    // Create a string with capacity
    let mut s = String::with_capacity(10);

    s.push('a');
    s.push('b');

    //Assertion testing. Check whether lhs=rhs. If true nothing happens. If false then at run time
    //an error is thrown
    assert_eq!(2, s.len());
}
