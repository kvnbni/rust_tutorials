pub fn run(){
    greet("Kevin","Benni");
    println!("Sum: {}", sum(5,5));

    //Closure function. You can use outside variables.
    let c: i32 = 1;
    let difference = |a:i32, b:i32| a-b-c;
    println!("Difference is {}", difference(5,3) );
}

fn greet(first_name: &str,last_name:&str){
    println!("Hello {} {}", first_name, last_name );
}

//Function returns an i32 value
fn sum(x: i32, y:i32) -> i32{
    x+y //No semicolon means retruning the value
}
