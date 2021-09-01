//Tuples group together values of different types
//Max 12 values

pub fn run() {
    let person: (&str,&str,i8) = ("Kevin", "Kerala", 29);

    println!("{} is from {} and is {}", person.0, person.1, person.2);
}
