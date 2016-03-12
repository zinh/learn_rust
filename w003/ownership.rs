// Rust will free allocated memory when reaching out of scope
fn foo(){
    let v = vec![1,2,3];
}

// Only one reference to a allocated memory at a time
fn bar(){
    let v = vec![1,2,3];
    let v2 = v;
    println!("v[0] = ", v[0]); // will raise error
}

// Even when passing to a function
fn take(v: Vec<i32>){
    // do something with v
}

fn another_func(){
    let v = vec![1,2,3];
    take(v);
    println!("v[0] = {}", v[0]); // this too will raise error
}

// Trait Copy
fn copy_var(){
    // i32 implement Copy trait
    let v1 = 1;
    // so that we can copy v1 memory
    // All primitive implement Copy trait
    let v2 = v1;
    println!("v1 = {}", v1); // will print v1 normally
}
