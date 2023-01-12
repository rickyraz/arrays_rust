// module contains functions for querying the size and alignment of types, initializing and manipulating memory.
// Returns the size of the pointed-to value in bytes.
use std::mem::size_of_val;

fn main() {
    // arrays_main();
    // vectors();
    slices();
}

fn arrays_main() {
    let arr: [i32; 6] = [1, 2, 3, 4, 5, 9];
    let arr2: [i32; 4] = [8, 2, 7, 4];

    //print 24 (size of 6 --> 4-byte integers)
    print_array_size(arr);

    //print 16 (size of 4 --> 4-byte integers)
    println!("Array size in main function: {}", size_of_val(&arr2));

    //-------------------- !

    // let arr3: [i32; 3] = [1, 2, 3];
    // let index = 5;
    // //arr[index] panics !! with the following message:
    // //index out of bounds: the len is 3 but the index is 5
    // println!("Integer at index {}: {}", index, arr3[index]);
}

fn print_array_size(arr: [i32; 6]) {
    //print 24 (size of 6 --> 4-byte integers)

    println!(
        "Array size in print_array_size function: {}",
        size_of_val(&arr)
    );
}

fn vectors() {
    let mut v: Vec<i32> = vec![1, 2, 3, 4];

    //prints 4
    println!("v's capacity is {}", v.capacity());
    println!("Address of v's first element: {:p}", &v[0]); //{:p} prints the address

    v.push(5);

    //prints 8 -- KENAPA DAPET 8 ????
    println!("v's capacity is {}", v.capacity());
    println!("Address of v's first element: {:p}", &v[0]);
}

fn slices() {
    let arr: [i32; 5] = [10, 20, 30, 40, 50];

    // array ke 2 sampai ke 5 dari  =>> 0 = 10, 1 = 20, 2 = 30, 3 = 40, 4 = 50
    // 0 = 30, 1 = 40, 2 = 50
    let s = &arr[2..5];

    //prints 30
    println!("First element in slice: {:}", s[0]);
    //prints 50
    println!("third element in slice: {:}", s[2]);

    // 0 = 1 , 1 = 2, 2 = 3, 3 = 4
    let v: Vec<i32> = vec![1, 2, 3, 4];

    // 0 = 2, 1 = 3, 2 = 4, 3 = 5
    let s2 = &v[1..3];

    println!("second element in slice: {:}", s2[0]);
}
