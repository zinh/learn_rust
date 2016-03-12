// implicit lifetime
fn foo(x: &i32){
}

// explicit lifetime
// 'a read: the lifetime a
// bar has one lifetime called a
// we can have multiple lifetime, ie: fn bar<'a, 'b>()
// in case of mut reference: (x: &'a mut i32)
fn bar<'a>(x: &'a i32){
}
