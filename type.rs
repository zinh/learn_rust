fn main(){
    let x: i32 = 1;
    let mut y: i32 = 32;
    let s: &str = "Hello world";
    y = y + 10;
    println!("x = {:?}, y = {}", x, y);
    println!("{}", s);
}

fn acc() -> i32{
    let programs = "+ + - * /";
    let mut accumulator = 0;
    for token in programs.chars(){
        match token {
            '+' => accumulator += 1;
            '-' => accumulator -= 1;
            '*' => accumulator *= 2;
            '/' => accumulator  /= 2;
            _ => {}
        }
    }
    println!("{} = {}", programs, accumulator);
}
