fn foo(v1: &Vec<i32>) -> i32{
    let v2 = v1;
    println!("{}", v2[1]);
    42
}

fn main(){
    let mut v1 = vec![10,11];
    foo(&v1);
    println!("v[0] = {}", v1[0]);
}
