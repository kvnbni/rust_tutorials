pub fn run() {
    println!("Hello World!!");

    //Basic formatting
    println!("{} loves to {}", "Kevin", "code");

    //Positional arguments
    println!(
        "{0} is from {1} and {0} loves to {2}",
        "Kevin", "Kerala", "code"
    );

    //Named arguments
    println!(
        "{name} loves to play {sport}",
        name = "Kevin",
        sport = "ping pong"
    );

    //Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    //Placeholder for debug trait
    println!("{:?}", (12, "Hello", false));

    //Basic math
    println!("10+10= {}", 10 + 10);
}
