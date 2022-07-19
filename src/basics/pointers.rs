//reference pointers - point to a resource in memory

pub fn run() {
    //primitive arrays
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    /*with non-primitives, if we assign another variable to a piece of data the first variable will no longer
    hold that value, youll need a reference (&) to point to the resource
    */

    let v1 = vec![1, 2, 3];
    let v2 = &v1;

    println!("{:?}", (&v1, v2));
}
