pub fn run() {
    let name = "Kevin";
    let mut age = 18;

    println!("My name is {} and I am {}", name, age);

    age = 29;

    println!("I am now {}", age);

    //Define constant. When you define one you have to specify the type as well.
    const ID: i32 = 001;

    println!("ID: {}", ID);

    //Assigning multiple vars
    let (my_name, my_age) = ("Kevin", 29);

    println!("{} is {}", my_name, my_age);
}
