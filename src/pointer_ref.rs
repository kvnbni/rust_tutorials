pub fn run(){
    //Defining arrays of primitive nature.
    let arr1 = [1,2,3];
    let arr2 = arr1;

    println!("{:?}",(arr1, arr2));

    //Copying vectors in the same way won't work as they are non-primitives. We need to use pointers
    //instead. & are called as shared references. These point to memory owned by some other value.
    //When a shared reference to a value is created it prevents direct mutation of the value.

    let vec1 = vec![1,2,3];
    let vec2 = &vec1;

    println!("{:?}", (&vec1,vec2) ); //Not entirely sure why it is &vec1 and not just vec.


}
