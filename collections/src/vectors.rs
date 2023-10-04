/*
In Rust, the Vec<T> type is designed to store values of the same type in a contiguous block of memory.
This means that the actual values are stored next to each other in RAM, not just the pointers.
When you create a Vec<T>, it allocates a chunk of memory to hold a certain number of elements of type T.
As you add elements to the Vec, they are stored in this allocated memory, one after the other.
If the Vec needs to grow beyond its current capacity, it will allocate a new, larger chunk of memory and copy the existing elements into the new memory block.
This process ensures that the elements are stored in a contiguous sequence in memory.
*/

pub fn introduction() {
    //Allocating a new vector on heap
    //Explicit type annotation is required here because we aren't inserting any values
    let _vec: Vec<i32> = Vec::new();

    //Creating a vector with initial values
    //Note that the type annotation is not required here because we are inserting values
    let _vec_from_values = vec![1, 2, 3];

    //Pushing values to a vector
    let mut mutable_vec = Vec::new();
    mutable_vec.push(1);
    mutable_vec.push(2);
    mutable_vec.push(3);
    println!("mutable_vec: {:?}", mutable_vec);

    //By using loop
    let mut mutable_vec = Vec::new();
    let mut c = 0;
    loop {
        mutable_vec.push(c);
        c += 1;
        if c == 10 {
            break;
        }
    }
    println!("mutable_vec: {:?}", mutable_vec);

    //Reading values from a vector
    let third = mutable_vec.get(3);
    match third {
        Some(third) => println!("Element 3 is {}", third),
        None => println!("There is no element 3"),
    }

    mutable_vec.push(10);
    /*
    The code might look like it should work: why should a reference to the first element care about changes at the end of the vector?
    This error is due to the way vectors work: because vectors put the values next to each other in memory, adding a new element onto the end of the vector
    might require allocating new memory and copying the old elements to the new space,
    if there isn’t enough room to put all the elements next to each other where the vector is currently stored.
    In that case, the reference to the first element would be pointing to deallocated memory.
    The borrowing rules prevent programs from ending up in that situation.
    */
    //println!("third: {:?}", third.unwrap());

    //Iterating over values in a vector
    print!("mutable_vec: [ ");
    for i in &mutable_vec {
        print!("{} ", i);
    }
    println!("]");

    //Iterating and modifying values in a vector
    for i in &mut mutable_vec {
        *i += 10;
    }
    println!("mutable_vec: {:?}", mutable_vec);
}
//All vectors and their contents are freed when they go out of scope.
