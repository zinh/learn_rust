fn main(){
    let x: i32 = 1;
    let mut y: i32 = 32;
    let s: &str = "Hello world";
    y = y + 10;
    println!("x = {:?}, y = {}", x, y);
    println!("{}", s);
}
