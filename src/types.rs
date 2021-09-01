pub fn run() {
    //Define type explicitly
    let a: i64 = 1373127;

    //Finding max size of a type

    println!("Max size of i32 is {}", std::i32::MAX);

    //Boolean

    let is_greater: bool = 10 > 4;

    println!("{:?}", (a, is_greater));
}
