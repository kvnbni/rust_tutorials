//Vectors can be of variable size

pub fn run(){
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    //Printing entire vector
    println!("{:?}", numbers );

    //Printing single element
    println!("Position 0 of vector is {}", numbers[0]);

    //Length of vector is
    println!("Length of vector is {}", numbers.len());

    //Changing an element requires vector to be defined as mutable
    numbers[1] = 12;

    //Printing entire vector after change
    println!("{:?}", numbers );

    //Memory allocated to vector
    println!("Memory allocated to vector: {} bytes", std::mem::size_of_val(&numbers));

    //Get slice of an vector
    let slice: &[i32] = &numbers[1..3];
    println!("{:?}", slice );

    //Loop through vector values
    for number in numbers.iter(){
        println!("Number: {}", number );
    }

    //Loop and mut contents in vector
    for number in numbers.iter_mut(){
        *number = *number * 2;
    }

    println!("{:?}",numbers );
}
