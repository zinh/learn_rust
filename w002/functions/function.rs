fn fizzbuzz(n: u32){
    for i in 1..n{
        if divisible_by(i, 15){
            println!("FizzBuzz");
            continue;
        }
        if divisible_by(i, 3){
            println!("Fizz");
            continue;
        }
        if divisible_by(i, 5){
            println!("Buzz");
            continue;
        }
        println!("{}", i);
    }
}

fn divisible_by(lhs: u32, rhs: u32) -> bool{
    if rhs == 0{
        return false;
    }
    lhs % rhs == 0
}

fn main(){
    fizzbuzz(101);
}
