fn main(){
    fn function (i: i32) -> i32{ i + 1};
    let closure_annotated = |i: i32| -> i32 {i + 1};
    let closure_inferred = |i| i + 1;
    let i = 1;
    println!("function: {}", function(i));
    println!("closure annotated: {}", closure_annotated(i));
    println!("closure inferred: {}", closure_inferred(i));
    let salute = "Ich vohne in Vietnam";
    let print = || println!("{}", salute);
    print();
}
