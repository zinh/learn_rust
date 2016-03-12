// We can let foo borrow v1, v2 reference
fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32{
    // note that reference cannot be changed
    v2.push(10); // this will raise error
    return 42;
}

fn bar(){
    let v1 = vec![1,2,3];
    let v2 = vec![1,2,3];
    let answer = foo(&v1, &v2);
    // we can use v1, v2 here
}

// We can change passed variable using mutable reference
fn foo1(){
    let mut x = 5;
    {
        let y = &mut x;
        *y += 1;
    }
    println!("x = {}", x); // this will print 6
}

// Borrowing rule:
// one or more reference (&T) to a resource
// exactly one mutable reference (&mut T)
fn foo3(){
    let mut x = 5;
    let y = &mut x;        // -+ &mut borrow of x start here
                           //  |
    *y += 1;               //  |
                           //  |
    println!("x = {}", x); // -+- try to borrow x here
}                          // -+ &mut borrow of x ends here
