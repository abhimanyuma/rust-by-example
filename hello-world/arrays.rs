use std::mem;

// This function borrows a slice
fn analyse_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("This slice has {} elements.", slice.len());
}

fn main() {
    // Fixed-size array, Rust inference will figure out the type
    // but we are just being explicit, and will help in case we change
    // the array.
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // We can also initialize all array elements to the same value
    // This is similar to [0] * 500 in Python
    let ys: [i32; 500] = [0; 500];

    // Indexing starts at 0.
    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);

    // `len` returns the count of elements in the array.
    println!("Number of elements in array: {}", xs.len());

    // Since compilers know the type and the size of the array
    // and since it is fixed, arrays are stack allocated.
    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices.
    println!("Borrow the whole array as a slice.");
    analyse_slice(&xs);

    // Slices can point to a section of an array.
    // They are of the form [starting_index..ending_index].
    // `starting_index` is the first position in the slice.
    // `ending_index` is one more than the last position in the slice.
    println!("Borrow a section of the array as a slice.");
    // The borrowed slice only has 3 elements, since that is what is mentioned
    // and analyse_slice is unaware of the larger array from which this is
    // "sliced"
    analyse_slice(&ys[1 .. 4]);

    // Example of empty slice `&[]`:
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // Same but more verbose

    // Arrays can be safely accessed using `.get`, which returns an
    // `Option`. This can be matched as shown below, or used with
    // `.expect()` if you would like the program to exit with a nice
    // message instead of happily continue.
    for i in 0..xs.len() + 1 { // Oops, one element too far!
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }

}