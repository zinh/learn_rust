use std::mem;

fn analyze_slice(slice: &[i32]){
    println!("First element: {}", slice[0]);
    println!("Slice length: {}", slice.len());
}

fn main(){
    let xs:[i32;5] = [1,2,3,4,5];
    println!("Array length: {}", xs.len());
    println!("Array memory: {}", mem::size_of_val(&xs));
    // borrow entire array
    analyze_slice(&xs);
    // borrow a slice of array
    analyze_slice(&xs[1 .. 5]);
    // println!("{}", xs[5]);
}
