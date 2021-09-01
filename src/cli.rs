use std::env;

pub fn run(){
    let args: Vec<String> = env::args().collect(); //Storing the command line arguments to args

    println!("{:?}", args);

    let command = args[1].clone();

    println!("{}", command );


}
