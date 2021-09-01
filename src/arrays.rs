//Arrays are of fixed size and elements should be of the same type.

pub fn run(){
    let mut numbers: [i32;5] = [1,2,3,4,5];
    //Another way of defining an array
    //let arr1 = [1,2,3]; 

    //Printing entire array
    println!("{:?}", numbers );

    //Printing single element
    println!("Position 0 of array is {}", numbers[0]);

    //Length of array is
    println!("Length of array is {}", numbers.len());

    //Changing an element in array requires array to be defined as mutable
    numbers[1] = 12;

    //Printing entire array after change
    println!("{:?}", numbers );

    //Memory allocated to array
    println!("Memory allocated to array: {} bytes", std::mem::size_of_val(&numbers));

    //Get slice of an array
    let slice: &[i32] = &numbers[1..3];
    println!("{:?}", slice );
}
